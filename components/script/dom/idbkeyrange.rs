use crate::dom::bindings::codegen::Bindings::IDBKeyRangeBinding;
use crate::dom::bindings::codegen::Bindings::IDBKeyRangeBinding::IDBKeyRangeMethods;
use crate::dom::bindings::error::{Error, ErrorResult};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{DomRoot, MutDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::globalscope::GlobalScope;
use crate::dom::window::Window;

use dom_struct::dom_struct;

use crate::script_runtime::JSContext as SafeJSContext;
use js::jsapi::Heap;
use js::jsval::JSVal;
use js::rust::HandleValue;

#[dom_struct]
pub struct IDBKeyRange {
    reflector_: Reflector,
    #[ignore_malloc_size_of = "mozjs"]
    lower: Heap<JSVal>,
    #[ignore_malloc_size_of = "mozjs"]
    upper: Heap<JSVal>,
    lower_open: bool,
    upper_open: bool,
}

impl IDBKeyRange {
    pub fn new_inherited() -> IDBKeyRange {
        unimplemented!();
    }

    pub fn new(global: &Window) -> DomRoot<IDBKeyRange> {
        reflect_dom_object(
            Box::new(IDBKeyRange::new_inherited()),
            global,
            IDBKeyRangeBinding::Wrap,
        )
    }

    pub fn Only(
        cx: SafeJSContext,
        global: &DomRoot<GlobalScope>,
        value: HandleValue,
    ) -> DomRoot<IDBKeyRange> {
        unimplemented!();
    }

    pub fn LowerBound(
        cx: SafeJSContext,
        global: &DomRoot<GlobalScope>,
        lower: HandleValue,
        open: bool,
    ) -> DomRoot<IDBKeyRange> {
        unimplemented!();
    }

    pub fn UpperBound(
        cx: SafeJSContext,
        global: &DomRoot<GlobalScope>,
        upper: HandleValue,
        open: bool,
    ) -> DomRoot<IDBKeyRange> {
        unimplemented!();
    }

    pub fn Bound(
        cx: SafeJSContext,
        global: &DomRoot<GlobalScope>,
        lower: HandleValue,
        upper: HandleValue,
        lower_open: bool,
        upper_open: bool,
    ) -> DomRoot<IDBKeyRange> {
        unimplemented!();
    }

    pub fn Includes(cx: SafeJSContext, global: &Window, key: HandleValue) {
        unimplemented!();
    }
}

impl IDBKeyRangeMethods for IDBKeyRange {
    fn Lower(&self, cx: SafeJSContext) -> JSVal {
        unimplemented!();
    }

    fn Upper(&self, cx: SafeJSContext) -> JSVal {
        unimplemented!();
    }

    fn LowerOpen(&self) -> bool {
        unimplemented!();
    }

    fn UpperOpen(&self) -> bool {
        unimplemented!();
    }

    fn Includes(&self, cx: SafeJSContext, key: HandleValue) -> bool {
        unimplemented!();
    }
}
