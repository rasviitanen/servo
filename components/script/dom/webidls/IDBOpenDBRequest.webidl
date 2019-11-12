/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
 
[Pref="dom.indexeddb.enabled", Exposed=(Window,Worker)]
interface IDBOpenDBRequest : IDBRequest {
  // Event handlers:
  attribute EventHandler onblocked;
  attribute EventHandler onupgradeneeded;
};