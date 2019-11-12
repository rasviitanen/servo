/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

partial interface mixin WindowOrWorkerGlobalScope {
  [SameObject] readonly attribute IDBFactory indexedDB;
};

[Pref="dom.indexeddb.enabled", Exposed=(Window,Worker)]
interface IDBFactory {
  [NewObject, Throws] IDBOpenDBRequest open(DOMString name,
                                    optional [EnforceRange] unsigned long long version);
  [NewObject] IDBOpenDBRequest deleteDatabase(DOMString name);

  Promise<sequence<IDBDatabaseInfo>> databases();

  short cmp(any first, any second);
};

dictionary IDBDatabaseInfo {
  DOMString name;
  unsigned long long version;
};