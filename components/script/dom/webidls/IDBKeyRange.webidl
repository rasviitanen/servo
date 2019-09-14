/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
 
[Exposed=(Window,Worker)]
interface IDBKeyRange {
  readonly attribute any lower;
  readonly attribute any upper;
  readonly attribute boolean lowerOpen;
  readonly attribute boolean upperOpen;

  // Static construction methods:
  [NewObject] static IDBKeyRange only(any value);
  [NewObject] static IDBKeyRange lowerBound(any lower, optional boolean open = false);
  [NewObject] static IDBKeyRange upperBound(any upper, optional boolean open = false);
  [NewObject] static IDBKeyRange bound(any lower,
                                       any upper,
                                       optional boolean lowerOpen = false,
                                       optional boolean upperOpen = false);

  boolean _includes(any key);
};