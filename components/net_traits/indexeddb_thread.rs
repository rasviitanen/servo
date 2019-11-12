/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use ipc_channel::ipc::IpcSender;
use servo_url::ServoUrl;

#[derive(Debug, Deserialize, Serialize)]
pub enum IndexedDBThreadReturnType {
    Open(Option<u64>),
    NextSerialNumber(u64),
    StartTransaction(Result<(), ()>),
    Commit(Result<(), ()>),
    Version(u64),
    CreateObjectStore(Option<String>),
    UpgradeVersion(Result<u64, ()>),
    KVResult(Option<Vec<u8>>),
    Exit,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum IndexedDBTxnMode {
    Readonly,
    Readwrite,
    Versionchange,
}

// https://www.w3.org/TR/IndexedDB-2/#key-type
#[derive(Debug, Deserialize, Serialize)]
pub enum IndexedDBKeyType {
    Number(Vec<u8>),
    // TODO implement Date(),
    String(Vec<u8>),
    // TODO implement Binary(),
    // TODO implment Array(),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AsyncOperation {
    /// Gets the value associated with the given key in the associated idb data
    GetItem(
        IpcSender<IndexedDBThreadReturnType>,
        ServoUrl,
        String, // Store
        Vec<u8>,
    ),

    /// Sets the value of the given key in the associated idb data
    PutItem(
        IpcSender<IndexedDBThreadReturnType>,
        ServoUrl,
        String,           // Store
        IndexedDBKeyType, // Key
        Vec<u8>,          // Value
        bool,             // Should overwrite
    ),

    /// Removes the key/value pair for the given key in the associated idb data
    RemoveItem(
        IpcSender<IndexedDBThreadReturnType>,
        ServoUrl,
        String, // Store
        Vec<u8>,
    ),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SyncOperation {
    // Upgrades the version of the database
    UpgradeVersion(
        IpcSender<IndexedDBThreadReturnType>,
        u64, // Serial number for the transaction
        u64,
    ),

    // Checks if an object store has a key generator, used in e.g. Put
    HasKeyGenerator(
        IpcSender<bool>,
        ServoUrl,
        String, // Store
    ),

    // Commits changes of a transaction to the database
    Commit(
        IpcSender<IndexedDBThreadReturnType>,
        u64, // Transaction serial number
    ),

    // Creates a new store for the database
    CreateObjectStore(IpcSender<IndexedDBThreadReturnType>, ServoUrl, String, bool),

    Open(
        IpcSender<IndexedDBThreadReturnType>,
        ServoUrl,
        String,
        Option<u64>,
    ),

    // Returns an unique identifier that is used to be able to
    // commit/abort transactions.
    RegisterNewTxn(IpcSender<u64>),

    // Starts executing the requests of a transaction
    // https://www.w3.org/TR/IndexedDB-2/#transaction-start
    StartTransaction(
        IpcSender<IndexedDBThreadReturnType>,
        u64, // The serial number of the mutating transaction
        IndexedDBTxnMode,
    ),

    // Returns the version of the database
    Version(IpcSender<IndexedDBThreadReturnType>),

    /// Send a reply when done cleaning up thread resources and then shut it down
    Exit(IpcSender<IndexedDBThreadReturnType>),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum IndexedDBThreadMsg {
    Sync(SyncOperation),
    Async(
        u64, // Serial number of the transaction that requests this operation
        IndexedDBTxnMode,
        AsyncOperation,
    ),
}
