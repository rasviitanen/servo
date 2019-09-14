use crate::dom::bindings::codegen::Bindings::IDBTransactionBinding;
use crate::dom::bindings::codegen::Bindings::IDBTransactionBinding::IDBTransactionMethods;
use crate::dom::bindings::codegen::Bindings::IDBTransactionBinding::IDBTransactionMode;
use crate::dom::bindings::error::{Error, ErrorResult};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{Dom, DomRoot};
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::window::Window;
use crate::dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull;
use dom_struct::dom_struct;

use crate::dom::domexception::DOMException;
use crate::dom::domstringlist::DOMStringList;
use crate::dom::idbdatabase::IDBDatabase;
use crate::dom::idbobjectstore::IDBObjectStore;
use crate::dom::eventtarget::EventTarget;


use std::rc::Rc;

#[dom_struct]
pub struct IDBTransaction {
    eventtarget: EventTarget,
    object_store_names: Dom<DOMStringList>,
    mode: IDBTransactionMode,
    db: Dom<IDBDatabase>,
    error: Dom<DOMException>,

    #[ignore_malloc_size_of = "Rc"]
    onabort: Rc<EventHandlerNonNull>,
    #[ignore_malloc_size_of = "Rc"]
    oncomplete: Rc<EventHandlerNonNull>,
    #[ignore_malloc_size_of = "Rc"]
    onerror: Rc<EventHandlerNonNull>,
}

impl IDBTransaction {
    fn new_inherited() -> IDBTransaction {
        unimplemented!();
    }

    fn new(global: &Window) -> DomRoot<IDBTransaction> {
        unimplemented!();
    }
}

impl IDBTransactionMethods for IDBTransaction {
    fn Db(&self) -> DomRoot<IDBDatabase> {
        DomRoot::from_ref(&*self.db)
    }

    fn ObjectStore(&self, name: DOMString) -> DomRoot<IDBObjectStore> {
        unimplemented!();
    }

    fn Commit(&self) {
        unimplemented!();
    }

    fn Abort(&self) {
        unimplemented!();
    }

    fn ObjectStoreNames(&self) -> DomRoot<DOMStringList> { 
        unimplemented!();    
    }
    
    fn Mode(&self) -> IDBTransactionMode { 
        unimplemented!();    
    }

    fn Error(&self) -> DomRoot<DOMException> {
        unimplemented!();    
    }

    fn GetOnabort(&self) -> Option<Rc<EventHandlerNonNull>> {
        unimplemented!();
    }
    
    fn SetOnabort(&self, value: Option<Rc<EventHandlerNonNull>>) -> () { 
        unimplemented!();    
    }
    
    fn GetOncomplete(&self) -> Option<Rc<EventHandlerNonNull>> {
        unimplemented!();    
    }
    
    fn SetOncomplete(&self, value: Option<Rc<EventHandlerNonNull>>) -> () {
        unimplemented!();
    }
    
    fn GetOnerror(&self) -> Option<Rc<EventHandlerNonNull>> {
        unimplemented!();
    }
    
    fn SetOnerror(&self, value: Option<Rc<EventHandlerNonNull>>) -> () { 
        unimplemented!();
    }

}
