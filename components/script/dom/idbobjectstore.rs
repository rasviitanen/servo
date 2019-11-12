use crate::dom::bindings::codegen::Bindings::IDBCursorBinding::IDBCursorDirection;
use crate::dom::bindings::codegen::Bindings::IDBDatabaseBinding::IDBObjectStoreParameters;
use crate::dom::bindings::codegen::Bindings::IDBObjectStoreBinding;
use crate::dom::bindings::codegen::Bindings::IDBObjectStoreBinding::IDBIndexParameters;
use crate::dom::bindings::codegen::Bindings::IDBObjectStoreBinding::IDBObjectStoreMethods;
use crate::dom::bindings::codegen::UnionTypes::StringOrStringSequence;
use crate::dom::bindings::error::{Error, ErrorResult, Fallible};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::refcounted::Trusted;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{Dom, DomRoot, MutDom, MutNullableDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::event::{Event, EventBubbles, EventCancelable};
use crate::dom::window::Window;

use crate::dom::domstringlist::DOMStringList;
use dom_struct::dom_struct;

use crate::dom::bindings::structuredclone;
use crate::dom::idbindex::IDBIndex;
use crate::dom::idbrequest::IDBRequest;
use crate::dom::idbtransaction::IDBTransaction;

use crate::dom::bindings::codegen::Bindings::IDBTransactionBinding::IDBTransactionMode;
use crate::script_runtime::JSContext as SafeJSContext;
use js::conversions::ToJSValConvertible;
use js::jsapi::{
    ESClass, GetBuiltinClass, Heap, IsArrayBufferObject, JSObject, JS_GetOwnUCPropertyDescriptor,
    JS_GetStringLength, JS_IsArrayBufferViewObject, MutableHandle, PropertyDescriptor,
};
use js::jsval::JSVal;
use js::jsval::UndefinedValue;
use js::rust::{AutoObjectVectorWrapper, HandleValue, MutableHandleValue};

use ipc_channel::ipc::IpcSender;
use net_traits::indexeddb_thread::{
    AsyncOperation, IndexedDBKeyType, IndexedDBThreadMsg, SyncOperation,
};
use net_traits::IpcSend;
use profile_traits::ipc;
use script_traits::StructuredSerializedData;
use servo_url::ServoUrl;
use std::mem;

use servo_atoms::Atom;

use crate::dom::bindings::codegen::UnionTypes::IDBObjectStoreOrIDBIndexOrIDBCursor;

#[derive(JSTraceable, MallocSizeOf)]
pub enum KeyPath {
    String(DOMString),
    StringSequence(Vec<DOMString>),
}

impl From<&StringOrStringSequence> for KeyPath {
    fn from(path: &StringOrStringSequence) -> KeyPath {
        match path {
            StringOrStringSequence::String(inner) => KeyPath::String(inner.clone()),
            StringOrStringSequence::StringSequence(inner) => KeyPath::StringSequence(inner.clone()),
        }
    }
}

#[dom_struct]
pub struct IDBObjectStore {
    reflector_: Reflector,
    name: DOMString,
    key_path: Option<KeyPath>,
    index_names: DomRoot<DOMStringList>,
    transaction: MutNullableDom<IDBTransaction>,
    auto_increment: bool,
}

impl IDBObjectStore {
    pub fn new_inherited(
        global: &Window,
        name: DOMString,
        options: Option<&IDBObjectStoreParameters>,
    ) -> IDBObjectStore {
        let key_path: Option<KeyPath> = match options {
            Some(options) => options.keyPath.as_ref().map(|path| path.into()),
            None => None,
        };

        IDBObjectStore {
            reflector_: Reflector::new(),
            name: name,
            key_path: key_path,

            index_names: DOMStringList::new(global, Vec::new()),
            transaction: Default::default(),
            auto_increment: false,
        }
    }

    pub fn new(
        global: &Window,
        name: DOMString,
        options: Option<&IDBObjectStoreParameters>,
    ) -> DomRoot<IDBObjectStore> {
        reflect_dom_object(
            Box::new(IDBObjectStore::new_inherited(global, name, options)),
            global,
            IDBObjectStoreBinding::Wrap,
        )
    }

    fn get_url(&self) -> ServoUrl {
        self.global().get_url()
    }

    pub fn set_transaction(&self, transaction: &IDBTransaction) {
        self.transaction.set(Some(transaction));
    }

    pub fn transaction(&self) -> Option<DomRoot<IDBTransaction>> {
        self.transaction.get()
    }

    // https://www.w3.org/TR/IndexedDB-2/#valid-key-path
    pub fn is_valid_key_path(key_path: &StringOrStringSequence) -> bool {
        let is_valid = |path: &DOMString| {
            let path = path.to_string();
            let mut identifiers = path.split('.').into_iter();

            while let Some(identifier) = identifiers.next() {
                println!("##### IDENTIFIER ######");
                println!("#####    {:?}    ######", identifier);
                println!("##### ---------- ######");
            }

            true
        };

        match key_path {
            StringOrStringSequence::StringSequence(paths) => {
                if paths.is_empty() {
                    return false;
                }

                for path in paths {
                    if !is_valid(path) {
                        return false;
                    }
                }
                true
            },
            StringOrStringSequence::String(path) => is_valid(path),
        }
    }

    #[allow(unsafe_code)]
    // https://www.w3.org/TR/IndexedDB-2/#convert-value-to-key
    fn convert_value_to_key(
        cx: SafeJSContext,
        input: HandleValue,
        seen: Option<Vec<HandleValue>>,
    ) -> Result<IndexedDBKeyType, Error> {
        // Step 1: If seen was not given, then let seen be a new empty set.
        let seen = seen.unwrap_or(Vec::new());
        println!("-----------------------asd------------------");

        // Step 2: If seen contains input, then return invalid.
        // TODO Check if we have seen this key
        // Does not currently work with HandleValue,
        // as it does not implement PartialEq

        // Step 3
        // TODO Accept buffer, array and date as well
        if input.is_number() {
            println!("CONVERTING NUMBER VALUE TO KEY");

            // TODO check for NaN
            let key = structuredclone::write(cx, input, None).expect("Could not serialize key");
            return Ok(IndexedDBKeyType::Number(key.serialized.clone()));
        }

        if input.is_string() {
            println!("CONVERTING STRING VALUE TO KEY");

            let key = structuredclone::write(cx, input, None).expect("Could not serialize key");
            return Ok(IndexedDBKeyType::String(key.serialized.clone()));
        }

        if input.is_object() {
            rooted!(in(*cx) let object = input.to_object());
            unsafe {
                println!("EXECUTING UNSAFE CODE");

                let built_in_class: *mut ESClass =
                    libc::malloc(mem::size_of::<ESClass>()) as *mut ESClass;

                if built_in_class.is_null() {
                    panic!("failed to allocate memory");
                }

                if !GetBuiltinClass(*cx, object.handle().into(), built_in_class) {
                    libc::free(built_in_class as *mut libc::c_void);
                    return Err(Error::Data);
                }

                if let ESClass::Date = *built_in_class {
                    println!("CONVERTING DATE VALUE TO KEY");
                    // TODO
                }

                if IsArrayBufferObject(*object) || JS_IsArrayBufferViewObject(*object) {
                    println!("CONVERTING ARRAY BUFFER VALUE TO KEY");

                    let is_view_object = JS_IsArrayBufferViewObject(*object);

                    let key =
                        structuredclone::write(cx, input, None).expect("Could not serialize key");
                    libc::free(built_in_class as *mut libc::c_void);
                    // TODO Return the correct type here
                    return Ok(IndexedDBKeyType::Number(key.serialized.clone()));
                }

                if let ESClass::Array = *built_in_class {
                    println!("CONVERTING ARRAY VALUE TO KEY");
                    // TODO
                }

                libc::free(built_in_class as *mut libc::c_void);
            }
        }

        Err(Error::Data)
    }

    // https://www.w3.org/TR/IndexedDB-2/#evaluate-a-key-path-on-a-value
    #[allow(unsafe_code)]
    fn evaluate_key_path_on_value(
        cx: SafeJSContext,
        value: HandleValue,
        mut return_val: MutableHandleValue,
        key_path: &KeyPath,
    ) {
        rooted!(in(*cx) let mut target_object = UndefinedValue());
        let mut target_object_prop_name: String;

        match key_path {
            KeyPath::String(path) => {
                // Step 3
                let path_as_string = path.to_string();
                let mut tokenizer = path_as_string.split('.').into_iter().peekable();

                while let Some(token) = tokenizer.next() {
                    if token == "length" && value.is_string() {
                        println!("1. EVALUATION VALUE IS STRING");
                        rooted!(in(*cx) let input_val = value.to_string());
                        unsafe {
                            let string_len = JS_GetStringLength(*input_val) as f32;
                            string_len.to_jsval(*cx, return_val);
                        }
                        break;
                    }

                    if !value.is_object() {
                        // TODO
                        println!("1. EVALUATION VALUE IS NOT AN OBJECt");
                        return;
                    }

                    rooted!(in(*cx) let object = value.to_object());
                    rooted!(in(*cx) let mut desc = PropertyDescriptor::default());
                    rooted!(in(*cx) let mut intermediate = UndefinedValue());
                    let mut has_prop = false;

                    unsafe {
                        let prop_name_as_utf16: Vec<u16> = token.encode_utf16().collect();
                        let ok = JS_GetOwnUCPropertyDescriptor(
                            *cx,
                            object.handle().into(),
                            prop_name_as_utf16.as_ptr(),
                            token.len(),
                            desc.handle_mut().into(),
                        );

                        if !ok {
                            // TODO Handle this
                            println!("1. EVALUATION VALUE IS NOT OK");

                            return;
                        }

                        if !desc.handle().obj.is_null() {
                            println!("2. EVALUATION VALUE IS SET TO INTERMEDIATE");
                            *intermediate = desc.handle().value;
                            has_prop = true;
                        } else {
                            // If we get here it means the object doesn't have the property or the
                            // property is available throuch a getter. We don't want to call any
                            // getters to avoid potential re-entrancy.
                            // The blob object is special since its properties are available
                            // only through getters but we still want to support them for key
                            // extraction. So they need to be handled manually.
                        }

                        // let built_in_class: *mut ESClass =
                        // libc::malloc(mem::size_of::<ESClass>()) as *mut ESClass;

                        // if built_in_class.is_null() {
                        //     panic!("failed to allocate memory");
                        // }

                        // if !GetBuiltinClass(*cx, input_val.handle().into(), built_in_class) {
                        //     libc::free(built_in_class as *mut libc::c_void);
                        //     return Err(Error::Data)
                        // }
                    }
                    if has_prop {
                        println!("YAY WE HAVE PROP");

                        // Treat undefined as an error
                        if intermediate.is_undefined() {
                            // TODO Handle this
                            println!("UH OH INTERMEDIATE IS UNDEFINED");
                            return;
                        }

                        if let Some(ref next_token) = tokenizer.peek() {
                            println!("TODO HANDLE NEXT");
                        } else {
                            println!("SET RETURN HERE");
                            *return_val = *intermediate;
                        }
                    } else {
                        println!("TODO HANDLE TARGET OBJECT");

                        println!("SETTING TARGET OBJECT");
                        *target_object = *value;
                        target_object_prop_name = token.to_string();
                    }

                    if !target_object.is_undefined() {
                        // We have started inserting new objects or are about to just insert
                        // the first one.
                        println!("TARGET ObJECT IS SOMETHING! WHAT A GOOD THING!");
                        // return_val = UndefinedValue();
                    }
                }
            },
            KeyPath::StringSequence(paths) => {
                unimplemented!("String sequence keyPath is currently unsupported");
            },
        }
    }

    fn has_key_generator(&self) -> bool {
        let (sender, receiver) = ipc::channel(self.global().time_profiler_chan().clone()).unwrap();

        let operation =
            SyncOperation::HasKeyGenerator(sender, self.get_url(), self.name.to_string());

        self.global()
            .resource_threads()
            .sender()
            .send(IndexedDBThreadMsg::Sync(operation))
            .unwrap();

        receiver.recv().unwrap()
    }

    // https://www.w3.org/TR/IndexedDB-2/#extract-a-key-from-a-value-using-a-key-path
    fn extract_key(
        cx: SafeJSContext,
        input: HandleValue,
        key_path: &KeyPath,
        multi_entry: Option<bool>,
    ) -> Result<IndexedDBKeyType, Error> {
        // Step 1: Evaluate key path
        // TODO Do this propertly
        rooted!(in(*cx) let mut r = UndefinedValue());
        IDBObjectStore::evaluate_key_path_on_value(cx, input, r.handle_mut(), key_path);

        if let Some(multi_entry) = multi_entry {
            // TODO handle multi_entry cases
            unimplemented!("multiEntry keys are not yet supported");
        } else {
            IDBObjectStore::convert_value_to_key(cx, r.handle(), None)
        }
    }

    // https://www.w3.org/TR/IndexedDB-2/#object-store-in-line-keys
    fn uses_inline_keys(&self) -> bool {
        self.key_path.is_some()
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-put
    fn put(
        &self,
        cx: SafeJSContext,
        value: HandleValue,
        key: HandleValue,
        overwrite: bool,
    ) -> Fallible<DomRoot<IDBRequest>> {
        // Step 1: Let transaction be this object store handle's transaction.
        let transaction = self
            .transaction
            .get()
            .expect("No transaction in Object Store");

        // Step 2: Let store be this object store handle's object store.
        let store = self.name.to_string();

        // Step 3: If store has been deleted, throw an "InvalidStateError" DOMException.
        // FIXME:(rasviitanen)

        // Step 4-5: If transaction is not active, throw a "TransactionInactiveError" DOMException.
        if !transaction.is_active() {
            return Err(Error::TransactionInactive);
        }

        // Step 5: If transaction is a read-only transaction, throw a "ReadOnlyError" DOMException.
        match transaction.get_mode() {
            IDBTransactionMode::Readonly => {
                return Err(Error::ReadOnly);
            },
            _ => { /* Everything is fine */ },
        }

        // Step 6: If store uses in-line keys and key was given, throw a "DataError" DOMException.
        if !key.is_undefined() && self.uses_inline_keys() {
            return Err(Error::Data);
        }

        // Step 7: If store uses out-of-line keys and has no key generator
        // and key was not given, throw a "DataError" DOMException.
        if !self.uses_inline_keys() && !self.has_key_generator() && key.is_undefined() {
            return Err(Error::Data);
        }

        // Step 8: If key was given, then: convert a value to a key with key
        let mut serialized_key: IndexedDBKeyType;

        if !key.is_undefined() {
            serialized_key = IDBObjectStore::convert_value_to_key(cx, key, None)?;
        } else {
            // Step 11: We should use in-line keys instead
            if let Ok(kpk) = IDBObjectStore::extract_key(
                cx,
                value,
                self.key_path.as_ref().expect("No key path"),
                None,
            ) {
                serialized_key = kpk;
            } else {
                // FIXME:(rasviitanen)
                // Check if store has a key generator
                // Check if we can inject a key
                return Err(Error::Data);
            }
        }

        let serialized_value =
            structuredclone::write(cx, value, None).expect("Could not serialize value");

        let (sender, receiver) = ipc::channel(self.global().time_profiler_chan().clone()).unwrap();

        let request = IDBRequest::execute_async(
            receiver,
            IDBObjectStoreOrIDBIndexOrIDBCursor::IDBObjectStore(DomRoot::from_ref(&*self)),
            AsyncOperation::PutItem(
                sender,
                self.get_url(),
                store,
                serialized_key,
                serialized_value.serialized.clone(),
                overwrite,
            ),
            None,
        );

        Ok(request)
    }
}

impl IDBObjectStoreMethods for IDBObjectStore {
    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-put
    fn Put(
        &self,
        cx: SafeJSContext,
        value: HandleValue,
        key: HandleValue,
    ) -> Fallible<DomRoot<IDBRequest>> {
        self.put(cx, value, key, true)
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-add
    fn Add(
        &self,
        cx: SafeJSContext,
        value: HandleValue,
        key: HandleValue,
    ) -> Fallible<DomRoot<IDBRequest>> {
        self.put(cx, value, key, false)
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-delete
    fn Delete(&self, cx: SafeJSContext, query: HandleValue) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-clear
    fn Clear(&self) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-get
    fn Get(&self, cx: SafeJSContext, query: HandleValue) -> DomRoot<IDBRequest> {
        let store = self.name.to_string();

        let (sender, receiver) = ipc::channel(self.global().time_profiler_chan().clone()).unwrap();

        let serialized_query =
            structuredclone::write(cx, query, None).expect("Could not serialize value");

        IDBRequest::execute_async(
            receiver,
            IDBObjectStoreOrIDBIndexOrIDBCursor::IDBObjectStore(DomRoot::from_ref(&*self)),
            AsyncOperation::GetItem(
                sender,
                self.get_url(),
                store,
                serialized_query.serialized.clone(),
            ),
            None,
        )
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-getkey
    fn GetKey(&self, cx: SafeJSContext, query: HandleValue) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-getall
    fn GetAll(
        &self,
        cx: SafeJSContext,
        query: HandleValue,
        count: Option<u32>,
    ) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-getallkeys
    fn GetAllKeys(
        &self,
        cx: SafeJSContext,
        query: HandleValue,
        count: Option<u32>,
    ) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-count
    fn Count(&self, cx: SafeJSContext, query: HandleValue) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-opencursor
    fn OpenCursor(
        &self,
        cx: SafeJSContext,
        query: HandleValue,
        direction: IDBCursorDirection,
    ) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-openkeycursor
    fn OpenKeyCursor(
        &self,
        cx: SafeJSContext,
        query: HandleValue,
        direction: IDBCursorDirection,
    ) -> DomRoot<IDBRequest> {
        unimplemented!();
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-index
    fn Index(&self, name: DOMString) -> DomRoot<IDBIndex> {
        unimplemented!();
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-createindex
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

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-name
    fn Name(&self) -> DOMString {
        unimplemented!();
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-setname
    fn SetName(&self, value: DOMString) {
        unimplemented!();
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-keypath
    fn KeyPath(&self, cx: SafeJSContext) -> JSVal {
        unimplemented!();
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-indexnames
    fn IndexNames(&self) -> DomRoot<DOMStringList> {
        unimplemented!();
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-transaction
    fn Transaction(&self) -> DomRoot<IDBTransaction> {
        unimplemented!();
    }

    // https://www.w3.org/TR/IndexedDB-2/#dom-idbobjectstore-autoincrement
    fn AutoIncrement(&self) -> bool {
        unimplemented!();
    }
}
