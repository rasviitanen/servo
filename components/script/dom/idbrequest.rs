use crate::dom::bindings::codegen::Bindings::IDBRequestBinding;
use crate::dom::bindings::codegen::Bindings::IDBRequestBinding::IDBRequestMethods;
use crate::dom::bindings::codegen::Bindings::IDBRequestBinding::IDBRequestReadyState;
use crate::dom::bindings::codegen::Bindings::IDBTransactionBinding::IDBTransactionMode;
use crate::dom::bindings::codegen::UnionTypes::IDBObjectStoreOrIDBIndexOrIDBCursor;
use crate::dom::bindings::error::{Error, ErrorResult};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{Dom, DomRoot, MutDom, MutNullableDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::window::Window;
use crate::task_source::TaskSource;

use dom_struct::dom_struct;

use crate::compartments::enter_realm;
use crate::dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull;
use crate::dom::bindings::structuredclone;
use crate::dom::domexception::DOMException;
use crate::dom::eventtarget::EventTarget;
use crate::dom::idbcursor::IDBCursor;
use crate::dom::idbindex::IDBIndex;
use crate::dom::idbobjectstore::IDBObjectStore;
use crate::dom::idbtransaction::IDBTransaction;

use crate::script_runtime::JSContext as SafeJSContext;
use js::jsapi::Heap;
use js::jsapi::Value;
use js::jsval::JSVal;
use js::rust::HandleValue;
use servo_atoms::Atom;

use ipc_channel::ipc::IpcSender;
use net_traits::indexeddb_thread::{
    AsyncOperation, IndexedDBThreadMsg, IndexedDBThreadReturnType, IndexedDBTxnMode,
};
use net_traits::IpcSend;
use profile_traits::ipc;
use profile_traits::ipc::IpcReceiver;

use std::sync::mpsc;
use std::thread::Builder;

use js::jsval::UndefinedValue;
use script_traits::StructuredSerializedData;

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::domexception::DOMErrorName;
use std::cell::Cell;
use std::rc::Rc;

#[must_root]
#[derive(JSTraceable, MallocSizeOf)]
pub enum Source {
    ObjectStore(Dom<IDBObjectStore>),
    Index(Dom<IDBIndex>),
    Cursor(Dom<IDBCursor>),
}

impl From<IDBObjectStoreOrIDBIndexOrIDBCursor> for Source {
    #[allow(unrooted_must_root)]
    fn from(source: IDBObjectStoreOrIDBIndexOrIDBCursor) -> Source {
        match source {
            IDBObjectStoreOrIDBIndexOrIDBCursor::IDBObjectStore(store) => {
                Source::ObjectStore(Dom::from_ref(&*store))
            },
            IDBObjectStoreOrIDBIndexOrIDBCursor::IDBIndex(index) => {
                Source::Index(Dom::from_ref(&*index))
            },
            IDBObjectStoreOrIDBIndexOrIDBCursor::IDBCursor(cursor) => {
                Source::Cursor(Dom::from_ref(&*cursor))
            },
        }
    }
}

#[dom_struct]
pub struct IDBRequest {
    eventtarget: EventTarget,
    #[ignore_malloc_size_of = "mozjs"]
    result: Heap<JSVal>,
    error: MutNullableDom<DOMException>,
    source: DomRefCell<Option<Source>>,
    transaction: MutNullableDom<IDBTransaction>,
    ready_state: Cell<IDBRequestReadyState>,
}

impl IDBRequest {
    pub fn new_inherited() -> IDBRequest {
        IDBRequest {
            eventtarget: EventTarget::new_inherited(),

            result: Heap::default(),
            error: Default::default(),
            source: Default::default(),
            transaction: Default::default(),
            ready_state: Cell::new(IDBRequestReadyState::Pending),
        }
    }

    pub fn new(global: &Window) -> DomRoot<IDBRequest> {
        reflect_dom_object(
            Box::new(IDBRequest::new_inherited()),
            global,
            IDBRequestBinding::Wrap,
        )
    }

    pub fn set_source(&self, source: Option<IDBObjectStoreOrIDBIndexOrIDBCursor>) {
        std::mem::replace(
            &mut *self.source.borrow_mut(),
            source.map(|source| source.into()),
        );
    }

    pub fn is_err(&self) -> bool {
        // TODO this is probably wrong
        self.error.get().is_some()
    }

    pub fn set_ready_state_done(&self) {
        self.ready_state.set(IDBRequestReadyState::Done);
    }

    pub fn set_result(&self, result: HandleValue) {
        self.result.set(result.get());
    }

    pub fn set_error(&self, error: Error) {
        match error {
            Error::Version => {
                self.error.set(Some(&DOMException::new(
                    &self.global(),
                    DOMErrorName::VersionError,
                )));
            },
            _ => {},
        }
    }

    pub fn set_transaction(&self, transaction: &IDBTransaction) {
        self.transaction.set(Some(transaction));
    }

    pub fn dispatch_success(&self) {
        let global = self.global();
        let this = Trusted::new(self);
        global
            .as_window()
            .task_manager()
            .dom_manipulation_task_source()
            .queue(
                task!(send_success_notification: move || {
                    let this = this.root();
                    this.set_ready_state_done();
                    let global = this.global();
                    let event = Event::new(
                        &global,
                        Atom::from("success"),
                        EventBubbles::DoesNotBubble,
                        EventCancelable::NotCancelable,
                    );
                    event.upcast::<Event>().fire(this.upcast());
                }),
                global.upcast(),
            )
            .unwrap();
    }

    // https://www.w3.org/TR/IndexedDB-2/#asynchronously-execute-a-request
    pub fn execute_async(
        receiver: IpcReceiver<IndexedDBThreadReturnType>,
        source: IDBObjectStoreOrIDBIndexOrIDBCursor,
        operation: AsyncOperation,
        request: Option<DomRoot<IDBRequest>>,
    ) -> DomRoot<IDBRequest> {
        match source.into() {
            Source::ObjectStore(ref store) => {
                // Step 1: Let transaction be the transaction associated with source.
                let transaction = store.transaction().expect("Store has no transaction");
                let global = transaction.global();
                let window = global.as_window();

                // Step 2: Assert: transaction is active.
                if !transaction.is_active() {
                    // TODO: Make fallible
                    // return Err(Error::TransactionInactive);
                }

                // Step 3: If request was not given, let request be a new request with source as source.
                let request = request.unwrap_or({
                    let new_request = IDBRequest::new(window);
                    new_request.set_source(Some(
                        IDBObjectStoreOrIDBIndexOrIDBCursor::IDBObjectStore(DomRoot::from_ref(
                            &*store,
                        )),
                    ));
                    new_request.set_transaction(&transaction);
                    new_request
                });

                // Step 4: Add request to the end of transaction’s request list.
                transaction.add_request(&request);

                // Step 5: Run the operation, and queue a returning task in parallel
                // the result will be put into `receiver`
                let transaction_mode = match transaction.get_mode() {
                    IDBTransactionMode::Readonly => IndexedDBTxnMode::Readonly,
                    IDBTransactionMode::Readwrite => IndexedDBTxnMode::Readwrite,
                    IDBTransactionMode::Versionchange => IndexedDBTxnMode::Versionchange,
                };

                transaction
                    .global()
                    .resource_threads()
                    .sender()
                    .send(IndexedDBThreadMsg::Async(
                        transaction.get_serial_number(),
                        transaction_mode,
                        operation,
                    ))
                    .unwrap();

                let (task_source, canceller) = window
                    .task_manager()
                    .dom_manipulation_task_source_with_canceller();

                let trusted_request = Trusted::new(&*request);
                Builder::new()
                    .name("IndexedDBWaitForResult".to_owned())
                    .spawn(move || {
                        // FIXME:(rasviitanen) This should not be queued as a task,
                        // but in order for `Trusted::root` to work, we execute the
                        // task on the main thread after all
                        // The `recv()` to wait for the result is still running on this thread, so
                        // we won't block the main thread from executing.

                        if let IndexedDBThreadReturnType::KVResult(result) =
                            receiver.recv().expect("Channel dropped")
                        {
                            let _ = task_source.queue_with_canceller(
                            task!(wait_for_database_result: move || {
                                let trusted_request = trusted_request.root();
                                let global = trusted_request.global();
                                let window = global.as_window();
                                let cx = window.get_cx();

                                trusted_request.set_ready_state_done();

                                let _ac = enter_realm(&*trusted_request);
                                rooted!(in(*cx) let mut answer = UndefinedValue());

                                if let Some(serialized_data) = result {
                                    let data = StructuredSerializedData {
                                        serialized: serialized_data,
                                        ports: None,
                                    };

                                    if let Err(_) =
                                        structuredclone::read(&global, data, answer.handle_mut())
                                    {
                                        warn!("Error reading structuredclone data");
                                    }

                                    trusted_request.set_result(answer.handle());

                                    let transaction = trusted_request.transaction.get()
                                        .expect("Request unexpectedly has no transaction");

                                    let event = Event::new(
                                        &global,
                                        Atom::from("success"),
                                        EventBubbles::DoesNotBubble,
                                        EventCancelable::NotCancelable,
                                    );

                                    transaction.set_active_flag(true);
                                    event.upcast::<Event>().fire(trusted_request.upcast());
                                    transaction.set_active_flag(false);
                                } else {
                                    trusted_request.set_result(answer.handle());

                                    // FIXME:(rasviitanen)
                                    // Set the error of request to result

                                    let transaction = trusted_request.transaction.get()
                                        .expect("Request has no transaction");

                                    let event = Event::new(
                                        &global,
                                        Atom::from("error"),
                                        EventBubbles::Bubbles,
                                        EventCancelable::Cancelable,
                                    );

                                    transaction.set_active_flag(true);
                                    event.upcast::<Event>().fire(trusted_request.upcast());
                                    transaction.set_active_flag(false);
                                }

                            }),
                            &canceller,
                        );
                        }
                    })
                    .expect("Failed to spawn thread");

                // Step 6
                request
            },
            Source::Index(ref index) => {
                unimplemented!();
            },
            Source::Cursor(ref cursor) => {
                unimplemented!();
            },
        }
    }
}

impl IDBRequestMethods for IDBRequest {
    fn Result(&self, cx: SafeJSContext) -> JSVal {
        self.result.get()
    }

    fn GetError(&self) -> Option<DomRoot<DOMException>> {
        self.error.get()
    }

    fn GetSource(&self) -> Option<IDBObjectStoreOrIDBIndexOrIDBCursor> {
        match *self.source.borrow() {
            Some(ref source) => Some(match source {
                Source::ObjectStore(store) => {
                    IDBObjectStoreOrIDBIndexOrIDBCursor::IDBObjectStore(DomRoot::from_ref(&*store))
                },
                Source::Index(index) => {
                    IDBObjectStoreOrIDBIndexOrIDBCursor::IDBIndex(DomRoot::from_ref(&*index))
                },
                Source::Cursor(cursor) => {
                    IDBObjectStoreOrIDBIndexOrIDBCursor::IDBCursor(DomRoot::from_ref(&*cursor))
                },
            }),
            None => None,
        }
    }

    fn GetTransaction(&self) -> Option<DomRoot<IDBTransaction>> {
        self.transaction.get()
    }

    fn ReadyState(&self) -> IDBRequestReadyState {
        self.ready_state.get()
    }

    event_handler!(success, GetOnsuccess, SetOnsuccess);

    event_handler!(error, GetOnerror, SetOnerror);
}
