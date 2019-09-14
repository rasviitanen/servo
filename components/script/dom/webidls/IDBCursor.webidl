/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

[Exposed=(Window,Worker)]
interface IDBCursor {
  readonly attribute (IDBObjectStore or IDBIndex) source;
  readonly attribute IDBCursorDirection direction;
  readonly attribute any key;
  readonly attribute any primaryKey;
  [SameObject] readonly attribute IDBRequest request;

  void advance([EnforceRange] unsigned long count);
  //void continue(optional any key); TODO: continue is a reserved keyword, we cannot generate this.
  void continuePrimaryKey(any key, any primaryKey);

  [NewObject] IDBRequest update(any value);
  [NewObject] IDBRequest delete();
};

enum IDBCursorDirection {
  "next",
  "nextunique",
  "prev",
  "prevunique"
};

/*
[Exposed=(Window,Worker)]
interface IDBCursorWithValue : IDBCursor {
  readonly attribute any value;
};
*/
