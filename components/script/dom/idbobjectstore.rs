use crate::dom::bindings::codegen::Bindings::IDBCursorBinding::IDBCursorDirection;
use crate::dom::bindings::codegen::Bindings::IDBObjectStoreBinding;
use crate::dom::bindings::codegen::Bindings::IDBObjectStoreBinding::IDBIndexParameters;
use crate::dom::bindings::codegen::Bindings::IDBObjectStoreBinding::IDBObjectStoreMethods;
use crate::dom::bindings::codegen::UnionTypes::StringOrStringSequence;
use crate::dom::bindings::error::{Error, ErrorResult};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{Dom, MutDom, DomRoot};
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::window::Window;

use crate::dom::domstringlist::DOMStringList;
use dom_struct::dom_struct;

use crate::dom::idbindex::IDBIndex;
use crate::dom::idbrequest::IDBRequest;
use crate::dom::idbtransaction::IDBTransaction;

use crate::script_runtime::JSContext as SafeJSContext;
use js::rust::HandleValue;
use js::jsval::JSVal;
use js::jsapi::Heap;

#[dom_struct]
pub struct IDBObjectStore {
    reflector_: Reflector,
    name: DOMString,
    #[ignore_malloc_size_of = "mozjs"]
    key_path: Heap<JSVal>,
    index_names: Dom<DOMStringList>,
    transaction: Dom<IDBTransaction>,
    auto_increment: bool,
}

impl IDBObjectStore {
    pub fn new_inherited() -> IDBObjectStore {
        unimplemented!();
    }

    pub fn new(global: &Window) -> DomRoot<IDBObjectStore> {
        reflect_dom_object(
            Box::new(IDBObjectStore::new_inherited()),
            global,
            IDBObjectStoreBinding::Wrap,
        )
    }
}

impl IDBObjectStoreMethods for IDBObjectStore {
    fn Put(&self, cx: SafeJSContext, value: HandleValue, key: HandleValue) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn Add(&self, cx: SafeJSContext, value: HandleValue, key: HandleValue) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn Delete(&self, cx: SafeJSContext, query: HandleValue) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn Clear(&self) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn Get(&self, cx: SafeJSContext, query: HandleValue) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn GetKey(&self, cx: SafeJSContext, query: HandleValue) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn GetAll(&self, cx: SafeJSContext, query: HandleValue, count: Option<u32>) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    fn GetAllKeys(&self, cx: SafeJSContext, query: HandleValue, count: Option<u32>) -> DomRoot<IDBRequest> {
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

    fn Index(&self, name: DOMString) -> DomRoot<IDBIndex> {
        unimplemented!();
    }

    fn CreateIndex(
        &self,
        name: DOMString,
        key_path: StringOrStringSequence,
        options: &IDBIndexParameters,
    ) -> DomRoot<IDBIndex> {
        unimplemented!();
    }

    fn DeleteIndex(&self, name: DOMString) {
        unimplemented!();
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
    
    fn IndexNames(&self) -> DomRoot<DOMStringList> {
        unimplemented!();    
    }
    
    fn Transaction(&self) -> DomRoot<IDBTransaction> {
        unimplemented!();    
    }
    
    fn AutoIncrement(&self) -> bool {
        unimplemented!();    
    }
    
}
