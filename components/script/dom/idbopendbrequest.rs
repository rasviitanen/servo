use crate::dom::bindings::codegen::Bindings::IDBOpenDBRequestBinding;
use crate::dom::bindings::codegen::Bindings::IDBOpenDBRequestBinding::IDBOpenDBRequestMethods;
use crate::dom::bindings::error::{Error, ErrorResult};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::window::Window;
use crate::dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull;

use crate::dom::eventtarget::EventTarget;
use dom_struct::dom_struct;
use crate::dom::idbrequest::IDBRequest;
use std::rc::Rc;

#[dom_struct]
pub struct IDBOpenDBRequest {
    idbrequest: IDBRequest,

    #[ignore_malloc_size_of = "Rc"]
    onblocked: Rc<EventHandlerNonNull>,
    #[ignore_malloc_size_of = "Rc"]
    onupgradeneeded: Rc<EventHandlerNonNull>,
}

impl IDBOpenDBRequest {
    pub fn new_inherited() -> IDBOpenDBRequest {
        unimplemented!();
    }

    pub fn new(global: &Window) -> DomRoot<IDBOpenDBRequest> {
        unimplemented!();
    }
}

impl IDBOpenDBRequestMethods for IDBOpenDBRequest {
    fn GetOnblocked(&self) -> Option<Rc<EventHandlerNonNull>> {
        unimplemented!();
    }

    fn SetOnblocked(&self, value: Option<Rc<EventHandlerNonNull>>) {
        unimplemented!();
    }

    fn GetOnupgradeneeded(&self) -> Option<Rc<EventHandlerNonNull>> {
        unimplemented!();
    }

     fn SetOnupgradeneeded(&self, value: Option<Rc<EventHandlerNonNull>>) {
         unimplemented!();
     }
}