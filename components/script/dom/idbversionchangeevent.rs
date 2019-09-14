use crate::dom::bindings::codegen::Bindings::IDBVersionChangeEventBinding;
use crate::dom::bindings::codegen::Bindings::IDBVersionChangeEventBinding::IDBVersionChangeEventMethods;
use crate::dom::bindings::codegen::Bindings::IDBVersionChangeEventBinding::IDBVersionChangeEventInit;
use crate::dom::bindings::error::{Error, ErrorResult};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::window::Window;
use crate::dom::globalscope::GlobalScope;

use dom_struct::dom_struct;

#[dom_struct]
pub struct IDBVersionChangeEvent {
    event: Event,
    old_version: u64,
    new_version: Option<u64>,
}

impl IDBVersionChangeEvent {
    pub fn new_inherited() -> IDBVersionChangeEvent {
        unimplemented!();
    }

    pub fn new(global: &Window) -> DomRoot<IDBVersionChangeEvent> {
        reflect_dom_object(
            Box::new(IDBVersionChangeEvent::new_inherited()),
            global,
            IDBVersionChangeEventBinding::Wrap,
        )
    }

    pub fn Constructor(global: &DomRoot<GlobalScope>, arg0: DOMString, arg1: &IDBVersionChangeEventInit) -> Result<DomRoot<IDBVersionChangeEvent>, Error>{
        unimplemented!();
    }
}

impl IDBVersionChangeEventMethods for IDBVersionChangeEvent {
    fn OldVersion(&self) -> u64 {
        unimplemented!();
    }

    fn GetNewVersion(&self) -> Option<u64> {
        unimplemented!();
    }

    fn IsTrusted(&self) -> bool {
        unimplemented!();
    }

}