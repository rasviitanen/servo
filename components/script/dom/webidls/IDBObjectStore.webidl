/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

[Exposed=(Window,Worker)]
interface IDBObjectStore {
  attribute DOMString name;
  readonly attribute any keyPath;
  readonly attribute DOMStringList indexNames;
  [SameObject] readonly attribute IDBTransaction transaction;
  readonly attribute boolean autoIncrement;

  [NewObject] IDBRequest put(any value, optional any key);
  [NewObject] IDBRequest add(any value, optional any key);
  [NewObject] IDBRequest delete(any query);
  [NewObject] IDBRequest clear();
  [NewObject] IDBRequest get(any query);
  [NewObject] IDBRequest getKey(any query);
  [NewObject] IDBRequest getAll(optional any query,
                                optional [EnforceRange] unsigned long count);
  [NewObject] IDBRequest getAllKeys(optional any query,
                                    optional [EnforceRange] unsigned long count);
  [NewObject] IDBRequest count(optional any query);

  [NewObject] IDBRequest openCursor(optional any query,
                                    optional IDBCursorDirection direction = "next");
  [NewObject] IDBRequest openKeyCursor(optional any query,
                                       optional IDBCursorDirection direction = "next");

  IDBIndex index(DOMString name);

  [NewObject] IDBIndex createIndex(DOMString name,
                                   (DOMString or sequence<DOMString>) keyPath,
                                   optional IDBIndexParameters options = {});
  void deleteIndex(DOMString name);
};

dictionary IDBIndexParameters {
  boolean unique = false;
  boolean multiEntry = false;
};