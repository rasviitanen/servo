use crate::dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull;
use crate::dom::bindings::codegen::Bindings::IDBDatabaseBinding::IDBDatabaseMethods;
use crate::dom::bindings::codegen::Bindings::IDBDatabaseBinding::{self, IDBObjectStoreParameters};
use crate::dom::bindings::codegen::Bindings::IDBTransactionBinding::IDBTransactionMode;

use crate::dom::bindings::codegen::UnionTypes::StringOrStringSequence;
use crate::dom::bindings::error::{Error, ErrorResult, Fallible};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{Dom, DomRoot, MutNullableDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::eventtarget::EventTarget;
use crate::dom::window::Window;

use dom_struct::dom_struct;
use std::rc::Rc;

use crate::dom::domstringlist::DOMStringList;
use crate::dom::idbobjectstore::IDBObjectStore;
use crate::dom::idbopendbrequest::IDBOpenDBRequest;
use crate::dom::idbtransaction::IDBTransaction;
use crate::dom::idbversionchangeevent::IDBVersionChangeEvent;

use ipc_channel::ipc::IpcSender;
use net_traits::indexeddb_thread::{IndexedDBThreadMsg, IndexedDBThreadReturnType, SyncOperation};
use net_traits::IpcSend;
use profile_traits::ipc;

use js::rust::HandleValue;
use servo_url::origin::MutableOrigin;
use std::cell::Cell;

use crate::task_source::TaskSource;
use servo_atoms::Atom;

#[dom_struct]
pub struct IDBDatabase {
    eventtarget: EventTarget,
    name: DOMString,
    version: Cell<u64>,
    object_store_names: DomRoot<DOMStringList>,

    // No specification below this line
    upgrade_transaction: MutNullableDom<IDBTransaction>,

    // Flags
    closing: Cell<bool>,
}

impl IDBDatabase {
    pub fn new_inherited(
        global: &Window,
        origin: &MutableOrigin,
        name: DOMString,
        version: Option<u64>,
        request: &IDBOpenDBRequest,
    ) -> IDBDatabase {
        let version = version.unwrap_or(0);

        IDBDatabase {
            eventtarget: EventTarget::new_inherited(),
            name,
            version: Cell::new(version),
            object_store_names: DOMStringList::new(global, Vec::new()),

            upgrade_transaction: Default::default(),
            closing: Cell::new(false),
        }
    }

    pub fn new(
        global: &Window,
        origin: &MutableOrigin,
        name: DOMString,
        version: Option<u64>,
        request: &IDBOpenDBRequest,
    ) -> DomRoot<IDBDatabase> {
        reflect_dom_object(
            Box::new(IDBDatabase::new_inherited(
                global, origin, name, version, request,
            )),
            global,
            IDBDatabaseBinding::Wrap,
        )
    }

    fn get_idb_thread(&self) -> IpcSender<IndexedDBThreadMsg> {
        self.global().resource_threads().sender()
    }

    pub fn object_stores(&self) -> DomRoot<DOMStringList> {
        self.object_store_names.clone()
    }

    pub fn version(&self) -> u64 {
        let (sender, receiver) = ipc::channel(self.global().time_profiler_chan().clone()).unwrap();
        let operation = SyncOperation::Version(sender);

        self.get_idb_thread()
            .send(IndexedDBThreadMsg::Sync(operation))
            .unwrap();

        if let IndexedDBThreadReturnType::Version(version) = receiver.recv().unwrap() {
            version
        } else {
            unreachable!("Unexpected return type");
        }
    }

    pub fn set_transaction(&self, transaction: &IDBTransaction) {
        self.upgrade_transaction.set(Some(transaction));
    }

    pub fn transaction(&self) -> Option<DomRoot<IDBTransaction>> {
        self.upgrade_transaction.get()
    }

    pub fn dispatch_versionchange(&self, old_version: u64, new_version: Option<u64>) {
        let global = self.global();
        let this = Trusted::new(self);
        global
            .as_window()
            .task_manager()
            .dom_manipulation_task_source()
            .queue(
                task!(send_versionchange_notification: move || {
                    let this = this.root();
                    let global = this.global();
                    let event = IDBVersionChangeEvent::new(
                        &global,
                        Atom::from("versionchange"),
                        EventBubbles::DoesNotBubble,
                        EventCancelable::NotCancelable,
                        old_version,
                        new_version,
                    );
                    event.upcast::<Event>().fire(this.upcast());
                }),
                global.upcast(),
            )
            .unwrap();
    }
}

impl IDBDatabaseMethods for IDBDatabase {
    fn Transaction(
        &self,
        store_names: StringOrStringSequence,
        mode: IDBTransactionMode,
    ) -> DomRoot<IDBTransaction> {
        // Step 1: Check if upgrade transaction is running
        // TODO

        // Step 2: if close flag is set, throw error

        // Step 3
        match store_names {
            StringOrStringSequence::String(name) => IDBTransaction::new(
                &self.global().as_window(),
                &self,
                mode,
                DOMStringList::new(&self.global().as_window(), vec![name]),
            ),
            StringOrStringSequence::StringSequence(sequence) => {
                // TODO remove duplicates from sequence
                IDBTransaction::new(
                    &self.global().as_window(),
                    &self,
                    mode,
                    DOMStringList::new(&self.global().as_window(), sequence),
                )
            },
        }
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbdatabase-createobjectstore
    fn CreateObjectStore(
        &self,
        name: DOMString,
        options: &IDBObjectStoreParameters,
    ) -> Fallible<DomRoot<IDBObjectStore>> {
        // TODO The upgrade transaction should probably be kept in the IDB thread.
        // Step 2
        let upgrade_transaction = match self.upgrade_transaction.get() {
            Some(txn) => txn,
            None => return Err(Error::InvalidState),
        };

        // Step 3
        if !upgrade_transaction.is_active() {
            return Err(Error::TransactionInactive);
        }

        // Step 4
        let key_path = options.keyPath.as_ref();

        // Step 5
        if let Some(ref path) = key_path {
            if !IDBObjectStore::is_valid_key_path(path) {
                return Err(Error::Syntax);
            }
        }

        // Step 6 TODO
        // If an object store named name already exists in database throw a "ConstraintError" DOMException.

        // Step 7
        let auto_increment = options.autoIncrement;

        // Step 8
        if auto_increment == true {
            match key_path {
                Some(StringOrStringSequence::String(path)) => {
                    if path == "" {
                        return Err(Error::InvalidAccess);
                    }
                },
                Some(StringOrStringSequence::StringSequence(paths)) => {
                    return Err(Error::InvalidAccess);
                },
                None => {},
            }
        }

        // Step 9
        let object_store =
            IDBObjectStore::new(&self.global().as_window(), name.clone(), Some(options));
        object_store.set_transaction(&upgrade_transaction);

        let (sender, receiver) = ipc::channel(self.global().time_profiler_chan().clone()).unwrap();

        let operation = SyncOperation::CreateObjectStore(
            sender,
            self.global().get_url(),
            name.to_string(),
            auto_increment,
        );

        self.get_idb_thread()
            .send(IndexedDBThreadMsg::Sync(operation))
            .unwrap();

        Ok(object_store)
    }

    fn DeleteObjectStore(&self, name: DOMString) {
        unimplemented!();
    }

    fn Name(&self) -> DOMString {
        self.name.clone()
    }

    fn Version(&self) -> u64 {
        self.version()
    }

    fn ObjectStoreNames(&self) -> DomRoot<DOMStringList> {
        self.object_store_names.clone()
    }

    // https://www.w3.org/TR/IndexedDB-2/#closing-connection
    fn Close(&self) {
        // Step 1: Set the close pending flag of connection.
        self.closing.set(true);

        // Step 2: Handle force flag
        // TODO

        // Step 3: Wait for all transactions by this db to finish
        // TODO

        // Step 4: If force flag is set, fire a close event
        // TODO
    }

    event_handler!(abort, GetOnabort, SetOnabort);

    event_handler!(close, GetOnclose, SetOnclose);

    event_handler!(error, GetOnerror, SetOnerror);

    event_handler!(versionchange, GetOnversionchange, SetOnversionchange);
}
