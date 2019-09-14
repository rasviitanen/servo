use crate::dom::bindings::codegen::Bindings::IDBFactoryBinding::IDBFactoryMethods;
use crate::dom::bindings::codegen::Bindings::IDBFactoryBinding::{self, IDBDatabaseInfo};
use crate::dom::bindings::error::{Error, ErrorResult};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::window::Window;
use crate::dom::promise::Promise;

use crate::dom::idbopendbrequest::IDBOpenDBRequest;

use dom_struct::dom_struct;
use std::rc::Rc;

use crate::script_runtime::JSContext as SafeJSContext;
use js::rust::HandleValue;

#[dom_struct]
pub struct IDBFactory {
    reflector_: Reflector,
}

impl IDBFactory {
    pub fn new_inherited() -> IDBFactory {
        unimplemented!();
    }

    pub fn new(global: &Window) -> DomRoot<IDBFactory> {
        reflect_dom_object(Box::new(IDBFactory::new_inherited()), global, IDBFactoryBinding::Wrap)
    }
}

impl IDBFactoryMethods for IDBFactory {
    fn Open(&self, name: DOMString, version: Option<u64>) -> DomRoot<IDBOpenDBRequest> {
        unimplemented!();
    }

    fn DeleteDatabase(&self, name: DOMString) -> DomRoot<IDBOpenDBRequest> {
        unimplemented!();
    }

    fn Databases(&self) -> Rc<Promise> {
        unimplemented!();
    }

    fn Cmp(&self, cx: SafeJSContext, first: HandleValue, second: HandleValue) -> i16 {
        unimplemented!();
    }
}
