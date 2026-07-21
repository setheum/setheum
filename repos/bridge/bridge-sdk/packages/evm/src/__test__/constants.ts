// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

import type { Eip1193Provider } from '@buildwithsygma/core';

export const ASSET_TRANSFER_PARAMS = {
  source: 1,
  destination: 2,
  sourceAddress: '0x98729c03c4D5e820F5e8c45558ae07aE63F97461',
  sourceNetworkProvider: jest.fn() as unknown as Eip1193Provider,
  recipientAddress: '0x98729c03c4D5e820F5e8c45558ae07aE63F97461',
  resource: {
    address: '0x98729c03c4D5e820F5e8c45558ae07aE63F97461',
    resourceId: '0x0',
    caip19: '0x11',
  },
};
