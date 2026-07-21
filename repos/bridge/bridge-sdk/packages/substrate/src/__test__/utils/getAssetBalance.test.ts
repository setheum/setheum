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
import { Option, TypeRegistry } from '@polkadot/types';
import type { AssetBalance } from '@polkadot/types/interfaces';

import { getAssetBalance } from '../../utils/getAssetBalance.js';

const registry = new TypeRegistry();
const mockApi = {
  query: {
    assets: {
      account: jest.fn(),
    },
  },
} as unknown as ApiPromise;

const mockAssetBalance: AssetBalance = registry.createType('AssetBalance', {
  balance: registry.createType('u64', 1000),
  isFrozen: registry.createType('bool', false),
  isSufficient: registry.createType('bool', false),
});

describe('getAssetBalance', () => {
  let unwrapOrDefaultSpy: jest.SpyInstance;

  beforeEach(() => {
    (mockApi.query.assets.account as unknown as jest.Mock).mockReset();
    unwrapOrDefaultSpy = jest.spyOn(Option.prototype, 'unwrapOrDefault');
  });

  afterEach(() => {
    unwrapOrDefaultSpy.mockRestore();
  });

  it('should return the asset balance when present', async () => {
    const mockOption = new Option(registry, 'AssetBalance', mockAssetBalance);
    unwrapOrDefaultSpy.mockReturnValue(mockAssetBalance);
    (mockApi.query.assets.account as unknown as jest.Mock).mockResolvedValue(mockOption);

    const result = await getAssetBalance(mockApi, 1, 'accountAddress');
    expect(result).toEqual(mockAssetBalance);
    expect(mockApi.query.assets.account).toHaveBeenCalledWith(1, 'accountAddress');
    expect(unwrapOrDefaultSpy).toHaveBeenCalled();
  });
});
