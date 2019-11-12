pub use self::rkv::RkvEngine;

use std::collections::VecDeque;

use net_traits::indexeddb_thread::{AsyncOperation, IndexedDBTxnMode};
use tokio::prelude::Future;
use tokio::sync::oneshot::error::RecvError;

mod rkv;

pub struct StoreDescription {
    origin: String,
    name: String,
}

impl StoreDescription {
    pub fn new(origin: String, name: String) -> StoreDescription {
        StoreDescription { origin, name }
    }
}

impl std::string::ToString for StoreDescription {
    fn to_string(&self) -> String {
        format!("{}::{}", self.origin, self.name)
    }
}

pub struct KvsTransaction {
    pub mode: IndexedDBTxnMode,
    pub requests: VecDeque<AsyncOperation>,
}

pub trait KvsEngine: Clone + Send + 'static {
    fn create_store(&self, store: StoreDescription, auto_increment: bool);

    fn process_transaction(
        &self,
        transaction: KvsTransaction,
    ) -> Box<dyn Future<Item = Option<Vec<u8>>, Error = RecvError> + Send>;

    fn has_key_generator(&self, store: StoreDescription) -> bool;
}
