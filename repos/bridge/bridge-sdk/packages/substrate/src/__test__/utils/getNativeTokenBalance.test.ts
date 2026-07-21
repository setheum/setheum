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

import type { ApiPromise } from '@polkadot/api';
import type { AccountInfo } from '@polkadot/types/interfaces';
import { BN } from '@polkadot/util';

import { getNativeTokenBalance } from '../../utils/getNativeTokenBalance.js';

describe('getNativeTokenBalance', () => {
  let mockApi: ApiPromise;

  beforeEach(() => {
    mockApi = {
      query: {
        system: {
          account: jest.fn(),
        },
      },
    } as unknown as ApiPromise;
  });

  it('should return the native token balance for a given account address', async () => {
    const mockAccountInfo: AccountInfo = {
      nonce: 1,
      consumers: 1,
      providers: 1,
      sufficients: 0,
      data: {
        free: new BN(1000),
        reserved: new BN(0),
        miscFrozen: new BN(0),
        feeFrozen: new BN(0),
      },
    } as unknown as AccountInfo;

    (mockApi.query.system.account as unknown as jest.Mock).mockResolvedValue(mockAccountInfo);

    const accountAddress = '5D4sHK8XJ39BfG2FnWXWJff7gPDP5x4bq8uSDA8fjU8nQ2gT';
    const result = await getNativeTokenBalance(mockApi, accountAddress);

    expect(result).toEqual(mockAccountInfo.data);
    expect(mockApi.query.system.account).toHaveBeenCalledWith(accountAddress);
  });
});
