/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::indexeddb::engines::{KvsEngine, KvsTransaction, RkvEngine, StoreDescription};

use std::borrow::ToOwned;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::thread;
use std::cell::Cell;

use ipc_channel::ipc::{self, IpcReceiver, IpcSender};
use net_traits::indexeddb_thread::{
    IndexedDBThreadMsg, IndexedDBThreadReturnType, SyncOperation
};

use servo_url::ServoUrl;


pub trait IndexedDBThreadFactory {
    fn new(config_dir: Option<PathBuf>) -> Self;
}

impl IndexedDBThreadFactory for IpcSender<IndexedDBThreadMsg> {
    fn new(config_dir: Option<PathBuf>) -> IpcSender<IndexedDBThreadMsg> {
        let (chan, port) = ipc::channel().unwrap();

        let mut idb_dir = PathBuf::new();
        config_dir.map(|p| idb_dir.push(p));
        idb_dir.push("IndexedDB");

        thread::Builder::new()
            .name("IndexedDBManager".to_owned())
            .spawn(move || {
                IndexedDBManager::new(port, RkvEngine::new(idb_dir.as_path())).start();
            })
            .expect("Thread spawning failed");

        chan
    }
}

struct IndexedDBManager<E: KvsEngine> {
    port: IpcReceiver<IndexedDBThreadMsg>,
    engine: Option<E>,
    version: Cell<u64>,
    upgrade_transaction: Option<u64>,

    transactions: HashMap<u64, KvsTransaction>,
    serial_number_counter: u64,
}

impl<E: KvsEngine> IndexedDBManager<E> {
    fn new(port: IpcReceiver<IndexedDBThreadMsg>, db: E) -> IndexedDBManager<E> {
        IndexedDBManager {
            port: port,
            engine: Some(db),
            version: Cell::new(0),
            upgrade_transaction: None,

            transactions: HashMap::new(),
            serial_number_counter: 0,
        }
    }
}

impl<E: KvsEngine> IndexedDBManager<E> {
    fn start(&mut self) {
        loop {
            let message = self.port.recv().expect("No message");
            match message {
                IndexedDBThreadMsg::Sync(operation) => {
                    self.handle_sync_operation(operation);
                },
                IndexedDBThreadMsg::Async(txn, mode, operation) => {
                    // Queues an operation for a transaction without starting it
                    match self.transactions.entry(txn) {
                        Entry::Vacant(e) => {
                            let mut requests = VecDeque::new();
                            requests.push_back(operation);
                            e.insert(KvsTransaction { requests, mode });
                        },
                        Entry::Occupied(mut e) => {
                            e.get_mut().requests.push_back(operation);
                        },
                    }

                    // FIXME:(rasviitanen) schedule transactions properly
                    self.start_transaction(txn);
                },
            }
        }
    }

    fn handle_sync_operation(&mut self, operation: SyncOperation) {
        match operation {
            SyncOperation::Open(sender, url, name, version) => {
                self.open_db(sender, url, name, version);
            },
            SyncOperation::HasKeyGenerator(sender, url, store_name) => {
                let result = self.has_key_generator(url, store_name);
                sender.send(result).expect("Could not send generator info");
            },
            SyncOperation::Commit(sender, _txn) => {
                // FIXME:(rasviitanen) This does nothing at the moment
                sender
                    .send(IndexedDBThreadReturnType::Commit(Ok(())))
                    .expect("Could not send commit status");
            },
            SyncOperation::UpgradeVersion(sender, txn, version) => {
                self.upgrade_transaction = Some(txn);
                self.version.set(version);
                sender
                    .send(IndexedDBThreadReturnType::UpgradeVersion(Ok(self
                        .version
                        .get())))
                    .expect("Could not upgrade version");
            },
            SyncOperation::CreateObjectStore(sender, url, store_name, auto_increment) => {
                self.create_object_store(sender, url, store_name, auto_increment);
            },
            SyncOperation::StartTransaction(sender, txn, _mode) => {
                self.start_transaction(txn);
                sender
                    .send(IndexedDBThreadReturnType::StartTransaction(Ok(())))
                    .expect("Could not send start transition status");
            },
            SyncOperation::Version(sender) => {
                self.version(sender);
            },
            SyncOperation::RegisterNewTxn(sender) => {
                self.serial_number_counter += 1;
                sender
                    .send(self.serial_number_counter)
                    .expect("Could not send serial number");
            },
            SyncOperation::Exit(sender) => {
                // Nothing to do since we save idb set eagerly.
                let _ = sender.send(IndexedDBThreadReturnType::Exit).unwrap();
            },
        }
    }

    fn open_db(
        &mut self,
        _sender: IpcSender<IndexedDBThreadReturnType>,
        _url: ServoUrl,
        _name: String,
        version: Option<u64>,
    ) {
        if self.version.get() == 0 {
            self.version.set(version.unwrap_or(1));
        }
    }

    // Executes all requests for a transaction (without committing)
    fn start_transaction(&mut self, txn: u64) {
        // FIXME:(rasviitanen)
        // This executes in a thread pool, and `readwrite` transactions
        // will block their thread if the writer is occupied, so we can
        // probably do some smart things here in order to optimize.
        // Queueuing 8 writers will for example block the rest 7 threads,
        // so we should probably reserve write operations for just one thread,
        // so that the rest of the threads can work in parallel
        if let Some(transaction) = self.transactions.remove(&txn) {
            self.engine
                .as_ref()
                .unwrap()
                .process_transaction(transaction);
        };
    }

    fn has_key_generator(&self, url: ServoUrl, store_name: String) -> bool {
        let origin = self.origin_as_string(url);
        let store = StoreDescription::new(origin, store_name);

        self.engine.as_ref().unwrap().has_key_generator(store)
    }

    fn create_object_store(
        &mut self,
        _sender: IpcSender<IndexedDBThreadReturnType>,
        url: ServoUrl,
        store_name: String,
        auto_increment: bool,
    ) {
        let origin = self.origin_as_string(url);
        let store = StoreDescription::new(origin, store_name);

        self.engine
            .as_ref()
            .unwrap()
            .create_store(store, auto_increment);
    }

    fn version(&self, sender: IpcSender<IndexedDBThreadReturnType>) {
        sender
            .send(IndexedDBThreadReturnType::Version(self.version.get()))
            .expect("Could not get version");
    }

    fn origin_as_string(&self, url: ServoUrl) -> String {
        url.origin().ascii_serialization()
    }
}
