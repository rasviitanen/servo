/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

[Exposed=(Window,Worker),
 Constructor(DOMString type, optional IDBVersionChangeEventInit eventInitDict = {})]
interface IDBVersionChangeEvent : Event {
  readonly attribute unsigned long long oldVersion;
  readonly attribute unsigned long long? newVersion;
};

dictionary IDBVersionChangeEventInit : EventInit {
  unsigned long long oldVersion = 0;
  unsigned long long? newVersion = null;
};