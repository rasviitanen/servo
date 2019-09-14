use crate::dom::bindings::codegen::Bindings::IDBDatabaseBinding::IDBDatabaseMethods;
use crate::dom::bindings::codegen::Bindings::IDBDatabaseBinding::{self, IDBObjectStoreParameters};
use crate::dom::bindings::codegen::Bindings::IDBTransactionBinding::IDBTransactionMode;
use crate::dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull;

use crate::dom::bindings::codegen::UnionTypes::StringOrStringSequence;
use crate::dom::bindings::error::{Error, ErrorResult};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{Dom, DomRoot};
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::window::Window;
use crate::dom::eventtarget::EventTarget;

use dom_struct::dom_struct;
use std::rc::Rc;

use crate::dom::domstringlist::DOMStringList;
use crate::dom::idbobjectstore::IDBObjectStore;
use crate::dom::idbtransaction::IDBTransaction;
use js::rust::HandleValue;

#[dom_struct]
pub struct IDBDatabase {
    eventtarget: EventTarget,
    name: DOMString,
    version: u64,
    object_store_names: Dom<DOMStringList>,

    #[ignore_malloc_size_of = "Rc"]
    onabort: Rc<EventHandlerNonNull>,
    #[ignore_malloc_size_of = "Rc"]
    onclose: Rc<EventHandlerNonNull>,
    #[ignore_malloc_size_of = "Rc"]
    onerror: Rc<EventHandlerNonNull>,
    #[ignore_malloc_size_of = "Rc"]
    onversionchange: Rc<EventHandlerNonNull>,
}

impl IDBDatabase {
    pub fn new_inherited() -> IDBDatabase {
        unimplemented!();
    }

    pub fn new(global: &Window) -> DomRoot<IDBDatabase> {
        reflect_dom_object(
            Box::new(IDBDatabase::new_inherited()),
            global,
            IDBDatabaseBinding::Wrap,
        )
    }
}

impl IDBDatabaseMethods for IDBDatabase {
    fn Transaction(&self, store_names: StringOrStringSequence, mode: IDBTransactionMode) -> DomRoot<IDBTransaction> {
        unimplemented!();
    }

    fn CreateObjectStore(&self, name: DOMString, key_path: &IDBObjectStoreParameters) -> DomRoot<IDBObjectStore> {
        unimplemented!();
    }

    fn DeleteObjectStore(&self, name: DOMString) {
        unimplemented!();
    }

    fn Name(&self) -> DOMString {
        unimplemented!();    
    }
    
    fn Version(&self) -> u64 {
        unimplemented!();    
    }
    
    fn ObjectStoreNames(&self) -> DomRoot<DOMStringList> {
        unimplemented!();    
    }
    
    fn Close(&self) {
        unimplemented!();    
    }
    
    fn GetOnabort(&self) -> Option<Rc<EventHandlerNonNull>> {
        unimplemented!();    
    }
    
    fn SetOnabort(&self, value: Option<Rc<EventHandlerNonNull>>) {
        unimplemented!();    
    }
    
    fn GetOnclose(&self) -> Option<Rc<EventHandlerNonNull>> {
        unimplemented!();    
    }
    
    fn SetOnclose(&self, value: Option<Rc<EventHandlerNonNull>>) {
        unimplemented!();
    }
    
    fn GetOnerror(&self) -> Option<Rc<EventHandlerNonNull>> {
        unimplemented!();    
    }
    
    fn SetOnerror(&self, value: Option<Rc<EventHandlerNonNull>>) {
        unimplemented!();    
    }
    
    fn GetOnversionchange(&self) -> Option<Rc<EventHandlerNonNull>> {
        unimplemented!();
    }

    fn SetOnversionchange(&self, value: Option<Rc<EventHandlerNonNull>>) {
        unimplemented!();
    }
}
