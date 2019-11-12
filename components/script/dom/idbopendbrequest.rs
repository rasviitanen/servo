use crate::dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull;
use crate::dom::bindings::codegen::Bindings::IDBOpenDBRequestBinding;
use crate::dom::bindings::codegen::Bindings::IDBOpenDBRequestBinding::IDBOpenDBRequestMethods;
use crate::dom::bindings::codegen::Bindings::IDBRequestBinding::IDBRequestReadyState;
use crate::dom::bindings::error::{Error, ErrorResult};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{Dom, DomRoot, MutDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::eventtarget::EventTarget;
use crate::dom::extendableevent::ExtendableEvent;
use crate::dom::idbrequest::IDBRequest;
use crate::dom::window::Window;
use crate::task_source::TaskSource;
use dom_struct::dom_struct;
use js::rust::HandleValue;
use servo_atoms::Atom;
use std::rc::Rc;

use crate::compartments::enter_realm;
use crate::dom::idbdatabase::IDBDatabase;
use crate::dom::idbtransaction::IDBTransaction;
use crate::dom::idbversionchangeevent::IDBVersionChangeEvent;

use ipc_channel::ipc::IpcSender;
use net_traits::indexeddb_thread::{IndexedDBThreadMsg, IndexedDBThreadReturnType, SyncOperation};
use net_traits::IpcSend;
use profile_traits::ipc;
use profile_traits::ipc::IpcReceiver;

use std::sync::mpsc;

use crate::js::conversions::ToJSValConvertible;
use js::jsval::UndefinedValue;

#[dom_struct]
pub struct IDBOpenDBRequest {
    idbrequest: IDBRequest,
}

impl IDBOpenDBRequest {
    pub fn new_inherited() -> IDBOpenDBRequest {
        IDBOpenDBRequest {
            idbrequest: IDBRequest::new_inherited(),
        }
    }

    pub fn new(global: &Window) -> DomRoot<IDBOpenDBRequest> {
        reflect_dom_object(
            Box::new(IDBOpenDBRequest::new_inherited()),
            global,
            IDBOpenDBRequestBinding::Wrap,
        )
    }

    pub fn is_err(&self) -> bool {
        // TODO this is probably wrong
        self.idbrequest.is_err()
    }

    pub fn set_result(&self, result: HandleValue) {
        self.idbrequest.set_result(result);
    }

    pub fn set_error(&self, error: Error) {
        self.idbrequest.set_error(error);
    }

    pub fn set_transaction(&self, transaction: &IDBTransaction) {
        self.idbrequest.set_transaction(transaction);
    }

    #[allow(unsafe_code)]
    pub fn dispatch_success(&self, result: Trusted<IDBDatabase>) {
        let global = self.global();
        let this = Trusted::new(self);
        global
            .as_window()
            .task_manager()
            .dom_manipulation_task_source()
            .queue(
                task!(send_success_notification: move || {
                    let this = this.root();
                    let result = result.root();
                    this.idbrequest.set_ready_state_done();
                    let global = this.global();
                    let window = global.as_window();
                    let cx = window.get_cx();

                    let _ac = enter_realm(&*result);
                    rooted!(in(*cx) let mut result_val = UndefinedValue());
                    unsafe {
                        result.to_jsval(*cx, result_val.handle_mut());
                    }
                    this.set_result(result_val.handle());

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
            .expect("Could not queue success task");
    }

    pub fn dispatch_error(&self) {
        let global = self.global();
        let this = Trusted::new(self);
        global
            .as_window()
            .task_manager()
            .dom_manipulation_task_source()
            .queue(
                task!(send_success_notification: move || {
                    let this = this.root();
                    this.idbrequest.set_ready_state_done();
                    let global = this.global();
                    let event = Event::new(
                        &global,
                        Atom::from("error"),
                        EventBubbles::Bubbles,
                        EventCancelable::Cancelable,
                    );
                    event.upcast::<Event>().fire(this.upcast());
                }),
                global.upcast(),
            )
            .expect("Could not queue error tasks");
    }

    #[allow(unsafe_code)]
    pub fn dispatch_upgrade_transaction(
        &self,
        connection: &IDBDatabase,
        transaction: &IDBTransaction,
        old_version: u64,
        new_version: Option<u64>,
        completion_notificator: mpsc::Sender<Result<(), ()>>,
    ) {
        let global = self.global();
        let conn = Trusted::new(connection);
        let txn = Trusted::new(transaction);
        let this = Trusted::new(self);

        global
            .as_window()
            .task_manager()
            .dom_manipulation_task_source()
            .queue(
                task!(send_upgradeneeded_notification: move || {
                    let this = this.root();
                    let txn = txn.root();
                    let conn = conn.root();
                    let global = this.global();
                    let window = global.as_window();
                    let cx = window.get_cx();

                    let _ac = enter_realm(&*conn);
                    rooted!(in(*cx) let mut connection_val = UndefinedValue());
                    unsafe {
                        conn.to_jsval(*cx, connection_val.handle_mut());
                    }

                    this.idbrequest.set_result(connection_val.handle());
                    this.idbrequest.set_transaction(&txn);
                    this.idbrequest.set_ready_state_done();

                    let event = IDBVersionChangeEvent::new(
                        &global,
                        Atom::from("upgradeneeded"),
                        EventBubbles::DoesNotBubble,
                        EventCancelable::NotCancelable,
                        old_version,
                        new_version,
                    );

                    txn.set_active_flag(true);
                    let did_throw = event.upcast::<Event>().fire(this.upcast());
                    // TODO Handle throw (Step 8.5)
                    // https://www.w3.org/TR/IndexedDB-2/#run-an-upgrade-transaction
                    txn.set_active_flag(false);

                    // Notify that the upgrade is done
                    completion_notificator.send(Ok(())).unwrap();
                }),
                global.upcast(),
            )
            .expect("Could not queue task");
    }
}

impl IDBOpenDBRequestMethods for IDBOpenDBRequest {
    event_handler!(blocked, GetOnblocked, SetOnblocked);

    event_handler!(upgradeneeded, GetOnupgradeneeded, SetOnupgradeneeded);
}
