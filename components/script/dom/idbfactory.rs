use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::IDBFactoryBinding::IDBFactoryMethods;
use crate::dom::bindings::codegen::Bindings::IDBFactoryBinding::{self, IDBDatabaseInfo};
use crate::dom::bindings::error::{Error, ErrorResult, Fallible};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::promise::Promise;
use crate::dom::window::Window;

use crate::dom::bindings::codegen::Bindings::IDBTransactionBinding::IDBTransactionMode;
use crate::dom::idbdatabase::IDBDatabase;
use crate::dom::idbopendbrequest::IDBOpenDBRequest;
use crate::dom::idbrequest::IDBRequest;
use crate::dom::idbtransaction::IDBTransaction;

use crate::task_source::TaskSource;
use std::sync::mpsc;
use std::thread::Builder;

use crate::js::conversions::ToJSValConvertible;
use js::jsval::UndefinedValue;

use dom_struct::dom_struct;
use std::collections::VecDeque;
use std::rc::Rc;

use ipc_channel::ipc::IpcSender;
use net_traits::IpcSend;
use profile_traits::ipc;

use servo_url::origin::ImmutableOrigin;
use servo_url::origin::MutableOrigin;
use std::cell::RefCell;

use crate::script_runtime::JSContext as SafeJSContext;
use js::rust::HandleValue;

type ConnectionQueue = VecDeque<(MutableOrigin, DOMString)>;

#[dom_struct]
pub struct IDBFactory {
    reflector_: Reflector,

    queue: DomRefCell<ConnectionQueue>,
}

impl IDBFactory {
    pub fn new_inherited() -> IDBFactory {
        IDBFactory {
            reflector_: Reflector::new(),
            queue: DomRefCell::new(ConnectionQueue::new()),
        }
    }

    pub fn new(global: &Window) -> DomRoot<IDBFactory> {
        reflect_dom_object(
            Box::new(IDBFactory::new_inherited()),
            global,
            IDBFactoryBinding::Wrap,
        )
    }

    fn next_request(&self) -> Option<(MutableOrigin, DOMString)> {
        self.queue.borrow_mut().pop_front()
    }

    fn queue_request(&self, origin: MutableOrigin, name: DOMString) {
        self.queue.borrow_mut().push_back((origin, name));
    }

    fn open_internal(
        &self,
        name: DOMString,
        version: Option<u64>,
        request: &IDBOpenDBRequest,
        upgrade_notificator: mpsc::Sender<Result<(), ()>>,
        result_channel: mpsc::Sender<Result<Trusted<IDBDatabase>, Error>>,
    ) {
        let global = self.global();
        let origin = global.origin();
        let window = global.as_window();

        let result = IDBDatabase::new(window, &origin, name, version, &request);

        let db_version = result.version();
        let new_version = match version {
            Some(v) => v,
            None => {
                // db_version == 0 equals database == null
                if db_version == 0 {
                    1
                } else {
                    db_version
                }
            },
        };

        if new_version == db_version {
            upgrade_notificator.send(Ok(())).unwrap();
            result_channel.send(Ok(Trusted::new(&result))).unwrap();
        } else if new_version > db_version {
            // Step 5.10.4
            // TODO Do this for all open handles
            result.dispatch_versionchange(db_version, version);

            // Step 5.10.6
            // Dispatch an event to trigger version change
            let transaction = IDBTransaction::new(
                &self.global().as_window(),
                &result,
                IDBTransactionMode::Versionchange,
                result.object_stores(),
            );

            transaction.set_active_flag(false);
            transaction.start_and_upgrade_version(new_version); // Starts transaction

            result.set_transaction(&transaction);

            request.dispatch_upgrade_transaction(
                &result,
                &transaction,
                db_version,
                Some(new_version),
                upgrade_notificator,
            );

            result_channel.send(Ok(Trusted::new(&result))).unwrap();
        } else {
            upgrade_notificator.send(Ok(())).unwrap();
            result_channel.send(Err(Error::Version)).unwrap();
        }
    }
}

impl IDBFactoryMethods for IDBFactory {
    // https://www.w3.org/TR/IndexedDB-2/#dom-idbfactory-open
    #[allow(unsafe_code)]
    fn Open(&self, name: DOMString, version: Option<u64>) -> Fallible<DomRoot<IDBOpenDBRequest>> {
        // Step 1: If version is 0 (zero), throw a TypeError.
        if version == Some(0) {
            return Err(Error::Type(
                "The version must be an integer >= 1".to_owned(),
            ));
        };

        // Step 2: Let origin be the origin of the global scope used to
        // access this IDBFactory.
        let global = self.global();
        let origin = global.origin();
        let window = global.as_window();

        // Step 3: if origin is an opaque origin,
        // throw a "SecurityError" DOMException and abort these steps.
        if let ImmutableOrigin::Opaque(_) = origin.immutable() {
            return Err(Error::Security);
        }

        // Step 4: Let request be a new open request.
        let request = IDBOpenDBRequest::new(&self.global().as_window());

        // Step 5: Runs in parallel
        let (task_source, canceller) = window
            .task_manager()
            .dom_manipulation_task_source_with_canceller();

        let this = Trusted::new(self);
        let trusted_request = Trusted::new(&*request);
        let name_as_string = name.to_string(); // DOMString is not Send
        Builder::new()
            .name("IndexedDBOpenDatabase".to_owned())
            .spawn(move || {
                // FIXME:(rasviitanen) This should not be queued as a task,
                // but in order for `Trusted::root` to work, we execute the
                // task on the main thread after all...
                // The recv() is still running on this thread, so
                // we won't freeze the code.

                // Channel to get the database we open
                let (sender, receiver) = mpsc::channel();

                // Channel to get notified when an upgrade has completed
                let (upgrade_sender, upgrade_receiver) = mpsc::channel();

                let this_clone = this.clone();
                let trusted_request_clone = trusted_request.clone();
                let _t1 = task_source.queue_with_canceller(
                    task!(create_database: move || {
                        let this = this_clone.root();
                        let request = trusted_request_clone.root();
                        this.open_internal(DOMString::from_string(name_as_string), version, &request, upgrade_sender, sender)
                    }),
                    &canceller,
                );

                // Wait until the database has completed and commited eventual upgrades
                let _ = upgrade_receiver.recv().expect("Could not wait for upgrade to finish");
                let result = receiver.recv().expect("Could not wait for idb open");

                let _t2 = task_source.queue_with_canceller(
                    task!(set_request_result_to_database: move || {
                        let this = this.root();
                        let request = trusted_request.root();

                        match result {
                            Ok(db) => {
                                let global = this.global();
                                let window = global.as_window();
                                let cx = window.get_cx();
                                request.dispatch_success(db);
                            },
                            Err(dom_exception) => {
                                request.set_result(HandleValue::undefined());
                                request.set_error(dom_exception);
                            }
                        }
                    }),
                    &canceller
                );
            })
            .expect("Failed to spawn thread");

        // Step 6
        Ok(request)
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbfactory-deletedatabase
    fn DeleteDatabase(&self, name: DOMString) -> DomRoot<IDBOpenDBRequest> {
        let origin = self.global().origin();
        IDBOpenDBRequest::new(&self.global().as_window())
    }

    fn Databases(&self) -> Rc<Promise> {
        unimplemented!();
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbfactory-cmp
    fn Cmp(&self, cx: SafeJSContext, first: HandleValue, second: HandleValue) -> i16 {
        unimplemented!();
    }
}
