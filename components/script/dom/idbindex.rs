use crate::dom::bindings::codegen::Bindings::IDBCursorBinding::IDBCursorDirection;
use crate::dom::bindings::codegen::Bindings::IDBIndexBinding;
use crate::dom::bindings::codegen::Bindings::IDBIndexBinding::IDBIndexMethods;
use crate::dom::bindings::error::{Error, ErrorResult};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{Dom, DomRoot, MutDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::window::Window;

use crate::dom::idbobjectstore::IDBObjectStore;
use crate::dom::idbrequest::IDBRequest;

use dom_struct::dom_struct;

use crate::script_runtime::JSContext as SafeJSContext;
use js::jsapi::Heap;
use js::jsval::JSVal;
use js::rust::HandleValue;

#[dom_struct]
pub struct IDBIndex {
    reflector_: Reflector,
    name: DOMString,
    object_store: Dom<IDBObjectStore>,
    #[ignore_malloc_size_of = "mozjs"]
    key_path: Heap<JSVal>,
    multi_entry: bool,
    unique: bool,
}

impl IDBIndex {
    fn new_inherited() -> IDBIndex {
        unimplemented!();
    }

    fn new(global: &Window) -> DomRoot<IDBIndex> {
        reflect_dom_object(
            Box::new(IDBIndex::new_inherited()),
            global,
            IDBIndexBinding::Wrap,
        )
    }
}

impl IDBIndexMethods for IDBIndex {
    fn Get(&self, cx: SafeJSContext, query: HandleValue) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn GetKey(&self, cx: SafeJSContext, query: HandleValue) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn GetAll(
        &self,
        cx: SafeJSContext,
        query: HandleValue,
        count: Option<u32>,
    ) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn GetAllKeys(
        &self,
        cx: SafeJSContext,
        query: HandleValue,
        count: Option<u32>,
    ) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn Count(&self, cx: SafeJSContext, query: HandleValue) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn OpenCursor(
        &self,
        cx: SafeJSContext,
        query: HandleValue,
        direction: IDBCursorDirection,
    ) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn OpenKeyCursor(
        &self,
        cx: SafeJSContext,
        query: HandleValue,
        direction: IDBCursorDirection,
    ) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn ObjectStore(&self) -> DomRoot<IDBObjectStore> {
        DomRoot::from_ref(&*self.object_store)
    }

    fn Name(&self) -> DOMString {
        unimplemented!();
    }

    fn SetName(&self, value: DOMString) {
        unimplemented!();
    }

    fn KeyPath(&self, cx: SafeJSContext) -> JSVal {
        unimplemented!();
    }

    fn MultiEntry(&self) -> bool {
        unimplemented!();
    }

    fn Unique(&self) -> bool {
        unimplemented!();
    }
}
