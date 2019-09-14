/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

[Exposed=(Window,Worker)]
interface IDBDatabase : EventTarget {
  readonly attribute DOMString name;
  readonly attribute unsigned long long version;
  readonly attribute DOMStringList objectStoreNames;

  [NewObject] IDBTransaction transaction((DOMString or sequence<DOMString>) storeNames,
                                         optional IDBTransactionMode mode = "readonly");
  void close();

  [NewObject] IDBObjectStore createObjectStore(
    DOMString name,
    optional IDBObjectStoreParameters options = {});
  void deleteObjectStore(DOMString name);

  // Event handlers:
  attribute EventHandler onabort;
  attribute EventHandler onclose;
  attribute EventHandler onerror;
  attribute EventHandler onversionchange;
};

dictionary IDBObjectStoreParameters {
  (DOMString or sequence<DOMString>)? keyPath = null;
  boolean autoIncrement = false;
};