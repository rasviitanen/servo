use crate::dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull;
use crate::dom::bindings::codegen::Bindings::IDBTransactionBinding;
use crate::dom::bindings::codegen::Bindings::IDBTransactionBinding::IDBTransactionMethods;
use crate::dom::bindings::codegen::Bindings::IDBTransactionBinding::IDBTransactionMode;
use crate::dom::bindings::codegen::UnionTypes::IDBObjectStoreOrIDBIndexOrIDBCursor;
use crate::dom::bindings::error::{Error, ErrorResult, Fallible};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{Dom, DomRoot, MutNullableDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::window::Window;
use crate::task_source::TaskSource;

use crate::dom::bindings::structuredclone;
use dom_struct::dom_struct;

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::domexception::DOMException;
use crate::dom::domstringlist::DOMStringList;
use crate::dom::eventtarget::EventTarget;
use crate::dom::idbdatabase::IDBDatabase;
use crate::dom::idbobjectstore::IDBObjectStore;
use crate::dom::idbrequest::{IDBRequest, Source};

use ipc_channel::ipc::IpcSender;
use net_traits::indexeddb_thread::{
    AsyncOperation, IndexedDBThreadMsg, IndexedDBThreadReturnType, IndexedDBTxnMode, SyncOperation,
};
use net_traits::IpcSend;
use profile_traits::ipc;
use profile_traits::ipc::IpcReceiver;

use crate::dom::bindings::inheritance::HasParent;
use crate::dom::globalscope::GlobalScope;
use servo_atoms::Atom;

use std::cell::Cell;
use std::rc::Rc;

#[dom_struct]
pub struct IDBTransaction {
    eventtarget: EventTarget,
    object_store_names: DomRoot<DOMStringList>,
    mode: IDBTransactionMode,
    db: Dom<IDBDatabase>,
    error: MutNullableDom<DOMException>,

    // Not specified in WebIDL below this line
    // https://www.w3.org/TR/IndexedDB-2/#transaction-request-list
    requests: DomRefCell<Vec<Dom<IDBRequest>>>,
    // https://www.w3.org/TR/IndexedDB-2/#transaction-active-flag
    active: Cell<bool>,
    // https://www.w3.org/TR/IndexedDB-2/#transaction-finish
    finished: Cell<bool>,
    // An unique identifier, used to commit and revert this transaction
    serial_number: u64,
}

impl IDBTransaction {
    fn new_inherited(
        connection: &IDBDatabase,
        mode: IDBTransactionMode,
        scope: DomRoot<DOMStringList>,
        serial_number: u64,
    ) -> IDBTransaction {
        IDBTransaction {
            eventtarget: EventTarget::new_inherited(),
            object_store_names: scope,
            mode: mode,
            db: Dom::from_ref(connection),
            error: Default::default(),

            requests: Default::default(),
            active: Cell::new(true),
            finished: Cell::new(false),
            serial_number: serial_number,
        }
    }

    pub fn new(
        global: &Window,
        connection: &IDBDatabase,
        mode: IDBTransactionMode,
        scope: DomRoot<DOMStringList>,
    ) -> DomRoot<IDBTransaction> {
        let serial_number = IDBTransaction::register_new(&global.as_parent());
        reflect_dom_object(
            Box::new(IDBTransaction::new_inherited(
                connection,
                mode,
                scope,
                serial_number,
            )),
            global,
            IDBTransactionBinding::Wrap,
        )
    }

    // Registers a new transaction in the idb thread, and gets an unique serial number in return.
    // The serial number is used when placing requests against a transaction
    // and allows us to commit/abort transactions running in our idb thread.
    fn register_new(global: &GlobalScope) -> u64 {
        let (sender, receiver) = ipc::channel(global.time_profiler_chan().clone()).unwrap();

        global
            .resource_threads()
            .sender()
            .send(IndexedDBThreadMsg::Sync(SyncOperation::RegisterNewTxn(
                sender,
            )))
            .unwrap();

        receiver.recv().unwrap()
    }

    pub fn set_active_flag(&self, status: bool) {
        self.active.set(status)
    }

    pub fn is_active(&self) -> bool {
        self.active.get()
    }

    pub fn get_mode(&self) -> IDBTransactionMode {
        self.mode
    }

    pub fn get_serial_number(&self) -> u64 {
        self.serial_number
    }

    pub fn add_request(&self, request: &IDBRequest) {
        self.requests.borrow_mut().push(Dom::from_ref(request));
    }

    pub fn start_and_upgrade_version(&self, version: u64) {
        let (sender, receiver) = ipc::channel(self.global().time_profiler_chan().clone()).unwrap();
        let transaction_mode = match self.mode {
            IDBTransactionMode::Readonly => IndexedDBTxnMode::Readonly,
            IDBTransactionMode::Readwrite => IndexedDBTxnMode::Readwrite,
            IDBTransactionMode::Versionchange => IndexedDBTxnMode::Versionchange,
        };

        let start_operation =
            SyncOperation::StartTransaction(sender, self.serial_number, transaction_mode);
        self.get_idb_thread()
            .send(IndexedDBThreadMsg::Sync(start_operation))
            .unwrap();
        receiver.recv().unwrap();

        // Transaction is done, upgrade version
        // TODO is this correct? Should it be Async?
        let (sender, receiver) = ipc::channel(self.global().time_profiler_chan().clone()).unwrap();
        let upgrade_version_operation =
            SyncOperation::UpgradeVersion(sender, self.serial_number, version);
        self.get_idb_thread()
            .send(IndexedDBThreadMsg::Sync(upgrade_version_operation))
            .unwrap();
        receiver.recv().unwrap();
    }

    fn dispatch_complete(&self) {
        let global = self.global();
        let this = Trusted::new(self);
        global
            .as_window()
            .task_manager()
            .dom_manipulation_task_source()
            .queue(
                task!(send_complete_notification: move || {
                    let this = this.root();
                    let global = this.global();
                    let event = Event::new(
                        &global,
                        Atom::from("complete"),
                        EventBubbles::DoesNotBubble,
                        EventCancelable::NotCancelable,
                    );
                    event.upcast::<Event>().fire(this.upcast());
                }),
                global.upcast(),
            )
            .unwrap();
    }

    fn get_idb_thread(&self) -> IpcSender<IndexedDBThreadMsg> {
        self.global().resource_threads().sender()
    }
}

impl IDBTransactionMethods for IDBTransaction {
    fn Db(&self) -> DomRoot<IDBDatabase> {
        // DomRoot::from_ref(&*self.db)
        unimplemented!();
    }

    fn ObjectStore(&self, name: DOMString) -> DomRoot<IDBObjectStore> {
        // Step 1: Handle the case where transaction has finised
        // TODO

        // Step 2: Check that the object store exists
        // TODO

        // Step 3:
        // TODO Don't create a new transaction if we already have one
        // with the same name
        let object_store = IDBObjectStore::new(&self.global().as_window(), name.clone(), None);
        object_store.set_transaction(&self);

        object_store
    }

    fn Commit(&self) -> Fallible<()> {
        // Step 1
        // TODO, should we start, or just commit?
        let (sender, receiver) = ipc::channel(self.global().time_profiler_chan().clone()).unwrap();
        let start_operation = SyncOperation::Commit(sender, self.serial_number);

        self.get_idb_thread()
            .send(IndexedDBThreadMsg::Sync(start_operation))
            .unwrap();

        let result = receiver.recv().unwrap();

        // Step 2
        if let IndexedDBThreadReturnType::Commit(Err(result)) = result {
            // TODO also support Unknown error
            return Err(Error::QuotaExceeded);
        }

        // Step 3
        // TODO https://www.w3.org/TR/IndexedDB-2/#commit-a-transaction
        // Steps 3.1 and 3.3
        self.dispatch_complete();

        Ok(())
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbtransaction-abort
    fn Abort(&self) -> Fallible<()> {
        if self.finished.get() {
            return Err(Error::InvalidState);
        }

        self.active.set(false);

        Ok(())
    }

    fn ObjectStoreNames(&self) -> DomRoot<DOMStringList> {
        unimplemented!();
    }

    fn Mode(&self) -> IDBTransactionMode {
        unimplemented!();
    }

    fn Error(&self) -> DomRoot<DOMException> {
        unimplemented!();
    }

    event_handler!(abort, GetOnabort, SetOnabort);

    event_handler!(complete, GetOncomplete, SetOncomplete);

    event_handler!(error, GetOnerror, SetOnerror);
}
