use crate::dom::bindings::codegen::Bindings::IDBRequestBinding;
use crate::dom::bindings::codegen::Bindings::IDBRequestBinding::IDBRequestMethods;
use crate::dom::bindings::codegen::Bindings::IDBRequestBinding::IDBRequestReadyState;
use crate::dom::bindings::error::{Error, ErrorResult};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{Dom, DomRoot, MutDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::window::Window;
use crate::dom::bindings::codegen::UnionTypes::IDBObjectStoreOrIDBIndexOrIDBCursor;

use dom_struct::dom_struct;

use crate::dom::domexception::DOMException;
use crate::dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull;
use crate::dom::idbobjectstore::IDBObjectStore;
use crate::dom::idbtransaction::IDBTransaction;
use crate::dom::idbindex::IDBIndex;
use crate::dom::idbcursor::IDBCursor;
use crate::dom::eventtarget::EventTarget;

use crate::script_runtime::JSContext as SafeJSContext;
use js::rust::HandleValue;
use js::jsval::JSVal;
use js::jsapi::Heap;

use std::rc::Rc;

#[must_root]
#[derive(JSTraceable, MallocSizeOf)]
enum Source {
    ObjectStore(Dom<IDBObjectStore>),
    Index(Dom<IDBIndex>),
    Cursor(Dom<IDBCursor>),
}

#[dom_struct]
pub struct IDBRequest {
    eventtarget: EventTarget,
    #[ignore_malloc_size_of = "mozjs"]
    result: Heap<JSVal>,
    error: Option<DOMException>,
    source: Option<Source>,
    transaction: Option<IDBTransaction>,
    ready_state: IDBRequestReadyState,

    // Event handlers
    #[ignore_malloc_size_of = "Rc"]
    onsuccess: Rc<EventHandlerNonNull>,
    #[ignore_malloc_size_of = "Rc"]
    onerror: Rc<EventHandlerNonNull>,
}

impl IDBRequest {
    fn new_inherited() -> IDBRequest {
        unimplemented!();
    }

    fn new(global: &Window) -> DomRoot<IDBRequest> {
        reflect_dom_object(
            Box::new(IDBRequest::new_inherited()),
            global,
            IDBRequestBinding::Wrap,
        )
    }
}

impl IDBRequestMethods for IDBRequest {
    fn Result(&self, cx: SafeJSContext) -> JSVal {
        unimplemented!();
    }

    fn GetError(&self) -> Option<DomRoot<DOMException>> {
        unimplemented!();
    }
    
    fn GetSource(&self) -> Option<IDBObjectStoreOrIDBIndexOrIDBCursor> {
        unimplemented!();
    }
    
    fn GetTransaction(&self) -> Option<DomRoot<IDBTransaction>> {
        unimplemented!();    
    }
    
    fn ReadyState(&self) -> IDBRequestReadyState {
        unimplemented!();    
    }
    
    fn GetOnsuccess(&self) -> Option<Rc<EventHandlerNonNull>> {
        unimplemented!();    
    }
    
    fn SetOnsuccess(&self, value: Option<Rc<EventHandlerNonNull>>) {
        unimplemented!();    
    }
    
    fn GetOnerror(&self) -> Option<Rc<EventHandlerNonNull>> {
        unimplemented!();    
    }
    
    fn SetOnerror(&self, value: Option<Rc<EventHandlerNonNull>>) {
        unimplemented!();    
    }
    
}
