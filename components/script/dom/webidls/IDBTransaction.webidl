/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

[Exposed=(Window,Worker)]
interface IDBTransaction : EventTarget {
  readonly attribute DOMStringList objectStoreNames;
  readonly attribute IDBTransactionMode mode;
  [SameObject] readonly attribute IDBDatabase db;
  readonly attribute DOMException error;

  IDBObjectStore objectStore(DOMString name);
  void commit();
  void abort();

  // Event handlers:
  attribute EventHandler onabort;
  attribute EventHandler oncomplete;
  attribute EventHandler onerror;
};

enum IDBTransactionMode {
  "readonly",
  "readwrite",
  "versionchange"
};