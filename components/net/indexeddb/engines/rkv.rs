use super::{KvsEngine, KvsTransaction, StoreDescription};

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};

use crate::indexeddb::thread_pool::TransactionPool;

use tokio::prelude::Future;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;

use net_traits::indexeddb_thread::{
    AsyncOperation, IndexedDBKeyType, IndexedDBThreadReturnType, IndexedDBTxnMode,
};

use rkv::{Manager, Rkv, SingleStore, StoreOptions, Value};

#[derive(Clone)]
struct Store {
    inner: SingleStore,
    key_generator: Option<u64>,
}

#[derive(Clone)]
pub struct RkvEngine {
    rkv_handle: Arc<RwLock<Rkv>>,
    open_stores: Arc<RwLock<HashMap<String, Store>>>,
    pool: TransactionPool,
}

impl RkvEngine {
    pub fn new(path: &Path) -> Self {
        std::fs::create_dir_all(path).expect("Could not create OS directory for idb");
        let rkv_handle = Manager::singleton()
            .write()
            .expect("Could not get write lock")
            .get_or_create(path, Rkv::new)
            .expect("Could not create database with this origin");

        // FIXME:(rasviitanen) What is a reasonable number of threads?
        let concurrent_threads: u32 = 8;

        RkvEngine {
            rkv_handle,
            open_stores: Arc::new(RwLock::new(HashMap::new())),
            pool: TransactionPool::new(concurrent_threads)
                .expect("Could not create idb transaction thread pool"),
        }
    }
}

impl KvsEngine for RkvEngine {
    fn create_store(&self, description: StoreDescription, auto_increment: bool) {
        let rkv = self.rkv_handle.read().unwrap();
        let new_store = rkv
            .open_single(&*description.to_string(), StoreOptions::create())
            .unwrap();

        let key_generator = {
            if auto_increment {
                Some(0)
            } else {
                None
            }
        };

        let store = Store {
            inner: new_store,
            key_generator,
        };

        self.open_stores
            .write()
            .expect("Could not aquire lock")
            .insert(description.to_string(), store);
    }

    fn has_key_generator(&self, description: StoreDescription) -> bool {
        let stores = self
            .open_stores
            .read()
            .expect("Could not aquire read lock on stores");

        stores
            .get(&description.to_string())
            .expect("Store not found")
            .key_generator
            .is_some()
    }

    fn process_transaction(
        &self,
        transaction: KvsTransaction,
    ) -> Box<dyn Future<Item = Option<Vec<u8>>, Error = RecvError> + Send> {
        // FIXME:(rasviitanen) This piece of code is a mess,
        // it should probably be refactored, and `expects` needs
        // another look to assert that they are safe.
        let db_handle = self.rkv_handle.clone();
        let stores = self.open_stores.clone();

        let (tx, rx) = oneshot::channel();
        self.pool.spawn(move || {
            let db = db_handle
                .read()
                .expect("Could not aquire read lock on idb handle");
            let stores = stores.read().expect("Could not aquire read lock on stores");

            if let IndexedDBTxnMode::Readonly = transaction.mode {
                let reader = db.read().expect("Could not create reader for idb");
                for request in transaction.requests {
                    match request {
                        AsyncOperation::GetItem(sender, url, store_name, key) => {
                            let origin = url.origin().ascii_serialization();
                            let store = StoreDescription::new(origin, store_name);
                            let store =
                                stores.get(&store.to_string()).expect("Could not get store");
                            let result = store.inner.get(&reader, key).expect("Could not get item");

                            if let Some(Value::Blob(blob)) = result {
                                sender
                                    .send(IndexedDBThreadReturnType::KVResult(Some(blob.to_vec())))
                                    .unwrap();
                            } else {
                                sender
                                    .send(IndexedDBThreadReturnType::KVResult(None))
                                    .unwrap();
                            }
                        },
                        _ => {
                            // We cannot reach this, as checks are made earlier so that
                            // no modifying requests are executed on readonly transactions
                            unreachable!(
                                "Cannot execute modifying request with readonly transactions"
                            );
                        },
                    }
                }
            } else {
                // Aquiring a writer will block the thread if another `readwrite` transaction is active
                let mut writer = db.write().expect("Could not create writer for idb");
                for request in transaction.requests {
                    match request {
                        AsyncOperation::PutItem(sender, url, store_name, key, value, overwrite) => {
                            let origin = url.origin().ascii_serialization();
                            let store = StoreDescription::new(origin, store_name);
                            let store =
                                stores.get(&store.to_string()).expect("Could not get store");
                            let key = match key {
                                IndexedDBKeyType::String(inner) => inner,
                                IndexedDBKeyType::Number(inner) => inner,
                            };
                            if overwrite {
                                if store
                                    .inner
                                    .put(&mut writer, key.clone(), &Value::Blob(&value))
                                    .is_ok() {
                                        sender
                                            .send(IndexedDBThreadReturnType::KVResult(Some(key)))
                                            .unwrap();
                                    } else {
                                        sender
                                        .send(IndexedDBThreadReturnType::KVResult(None))
                                        .unwrap();
                                    }
                            } else {
                                // FIXME:(rasviitanen) We should be able to set some flags in
                                // `rkv` in order to do this without running a get request first
                                if store
                                    .inner
                                    .get(&writer, key.clone())
                                    .expect("Could not get item")
                                    .is_none()
                                {
                                    if store
                                        .inner
                                        .put(&mut writer, key.clone(), &Value::Blob(&value))
                                        .is_ok() {
                                            sender
                                                .send(IndexedDBThreadReturnType::KVResult(Some(key)))
                                                .unwrap();
                                        } else {
                                            sender
                                            .send(IndexedDBThreadReturnType::KVResult(None))
                                            .unwrap();
                                        }
                                } else {
                                    sender
                                        .send(IndexedDBThreadReturnType::KVResult(None))
                                        .unwrap();
                                }
                            }
                        },
                        AsyncOperation::GetItem(sender, url, store_name, key) => {
                            let origin = url.origin().ascii_serialization();
                            let store = StoreDescription::new(origin, store_name);
                            let store =
                                stores.get(&store.to_string()).expect("Could not get store");
                            let result = store.inner.get(&writer, key).expect("Could not get item");

                            if let Some(Value::Blob(blob)) = result {
                                sender
                                    .send(IndexedDBThreadReturnType::KVResult(Some(blob.to_vec())))
                                    .unwrap();
                            } else {
                                sender
                                    .send(IndexedDBThreadReturnType::KVResult(None))
                                    .unwrap();
                            }
                        },
                        AsyncOperation::RemoveItem(sender, url, store_name, key) => {
                            let origin = url.origin().ascii_serialization();
                            let store = StoreDescription::new(origin, store_name);
                            let store =
                                stores.get(&store.to_string()).expect("Could not get store");

                            if store.inner.delete(&mut writer, key.clone()).is_ok() {
                                sender
                                    .send(IndexedDBThreadReturnType::KVResult(Some(key)))
                                    .unwrap();
                            } else {
                                sender
                                    .send(IndexedDBThreadReturnType::KVResult(Some(key)))
                                    .unwrap();
                            }
                        },
                    }
                }

                writer.commit().expect("Failed to commit to database");
            }

            if tx.send(None).is_err() {
                warn!("IDBTransaction's execution channel is dropped");
            };
        });

        Box::new(rx)
    }
}
