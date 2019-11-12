use crate::dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use crate::dom::bindings::codegen::Bindings::IDBVersionChangeEventBinding;
use crate::dom::bindings::codegen::Bindings::IDBVersionChangeEventBinding::IDBVersionChangeEventInit;
use crate::dom::bindings::codegen::Bindings::IDBVersionChangeEventBinding::IDBVersionChangeEventMethods;
use crate::dom::bindings::error::{Error, ErrorResult};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::globalscope::GlobalScope;
use crate::dom::window::Window;
use servo_atoms::Atom;

use dom_struct::dom_struct;

#[dom_struct]
pub struct IDBVersionChangeEvent {
    event: Event,
    old_version: u64,
    new_version: Option<u64>,
}

impl IDBVersionChangeEvent {
    pub fn new_inherited(old_version: u64, new_version: Option<u64>) -> IDBVersionChangeEvent {
        IDBVersionChangeEvent {
            event: Event::new_inherited(),
            old_version: old_version,
            new_version: new_version,
        }
    }

    pub fn new_uninitialized(global: &GlobalScope) -> DomRoot<IDBVersionChangeEvent> {
        reflect_dom_object(
            Box::new(IDBVersionChangeEvent::new_inherited(0, None)),
            global,
            IDBVersionChangeEventBinding::Wrap,
        )
    }

    pub fn new(
        global: &GlobalScope,
        type_: Atom,
        bubbles: EventBubbles,
        cancelable: EventCancelable,
        old_version: u64,
        new_version: Option<u64>,
    ) -> DomRoot<IDBVersionChangeEvent> {
        let ev = reflect_dom_object(
            Box::new(IDBVersionChangeEvent::new_inherited(
                old_version,
                new_version,
            )),
            global,
            IDBVersionChangeEventBinding::Wrap,
        );
        {
            let event = ev.upcast::<Event>();
            event.init_event(type_, bool::from(bubbles), bool::from(cancelable));
        }
        ev
    }

    pub fn Constructor(
        global: &DomRoot<GlobalScope>,
        type_: DOMString,
        init: &IDBVersionChangeEventInit,
    ) -> Result<DomRoot<IDBVersionChangeEvent>, Error> {
        let bubbles = EventBubbles::from(init.parent.bubbles);
        let cancelable = EventCancelable::from(init.parent.cancelable);

        let event = IDBVersionChangeEvent::new(
            global,
            Atom::from(type_),
            bubbles,
            cancelable,
            init.oldVersion,
            init.newVersion,
        );
        Ok(event)
    }
}

impl IDBVersionChangeEventMethods for IDBVersionChangeEvent {
    fn OldVersion(&self) -> u64 {
        self.old_version
    }

    fn GetNewVersion(&self) -> Option<u64> {
        self.new_version
    }

    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}
