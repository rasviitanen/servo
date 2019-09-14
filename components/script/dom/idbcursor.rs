use crate::dom::bindings::codegen::Bindings::IDBCursorBinding;
use crate::dom::bindings::codegen::Bindings::IDBCursorBinding::IDBCursorDirection;
use crate::dom::bindings::codegen::Bindings::IDBCursorBinding::IDBCursorMethods;
use crate::dom::bindings::codegen::UnionTypes::IDBObjectStoreOrIDBIndex;
use crate::dom::bindings::error::{Error, ErrorResult};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::trace::JSTraceable;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{Dom, DomRoot, MutDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::window::Window;
use js::jsapi::Heap;

use dom_struct::dom_struct;

use crate::dom::idbobjectstore::IDBObjectStore;
use crate::dom::idbrequest::IDBRequest;
use crate::dom::idbindex::IDBIndex;

use crate::script_runtime::JSContext as SafeJSContext;
use js::rust::HandleValue;
use js::jsval::JSVal;

#[must_root]
#[derive(JSTraceable, MallocSizeOf)]
enum Source {
    ObjectStore(Dom<IDBObjectStore>),
    Index(Dom<IDBIndex>),
}

#[dom_struct]
pub struct IDBCursor {
    reflector_: Reflector,
    source: Source,
    direction: IDBCursorDirection,
    #[ignore_malloc_size_of = "mozjs"]
    key: Heap<JSVal>,
    #[ignore_malloc_size_of = "mozjs"]
    primary_key: Heap<JSVal>,
    request: Dom<IDBRequest>,
}

impl IDBCursor {
    pub fn new_inherited() -> IDBCursor {
        unimplemented!();
    }

    pub fn new(global: &Window) -> DomRoot<IDBCursor> {
        reflect_dom_object(
            Box::new(IDBCursor::new_inherited()),
            global,
            IDBCursorBinding::Wrap,
        )
    }
}

impl IDBCursorMethods for IDBCursor {
    fn Update(&self, cx: SafeJSContext, value: HandleValue) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn Delete(&self) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn Request(&self) -> DomRoot<IDBRequest> {
        DomRoot::from_ref(&*self.request)
    }

    fn Advance(&self, count: u32) {
        unimplemented!();
    }

    /* TODO add this to webidl
    fn Continue(key: Option<HandleValue>) {
        unimplemented!();
    }
    */

    fn ContinuePrimaryKey(&self, cx: SafeJSContext, key: HandleValue, primary_key: HandleValue) {
        unimplemented!();
    }

    fn Source(&self) -> IDBObjectStoreOrIDBIndex {
        unimplemented!();
    }

    fn Direction(&self) -> IDBCursorDirection {
        unimplemented!();
    }
    
    fn Key(&self, cx: SafeJSContext) -> JSVal {
        unimplemented!();
    }
    
    fn PrimaryKey(&self, cx: SafeJSContext) -> JSVal {
        unimplemented!();
    }
    
}
