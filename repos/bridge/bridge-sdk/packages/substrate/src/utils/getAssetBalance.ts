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
import type { Option } from '@polkadot/types';
import type { AssetBalance } from '@polkadot/types/interfaces';

/**
 * Retrieves the asset balance of a given account.
 *
 * @category Token interactions
 * @param {ApiPromise} api - The API instance used to query the chain.
 * @param {number} assetID - The ID of the asset to query. {@link https://github.com/sygmaprotocol/sygma-substrate-pallets#multiasset | More details}
 * @param {string} accountAddress - The address of the account for which to retrieve the asset balance.
 * @returns {Promise<AssetBalance>} A promise that resolves with the retrieved asset balance.
 */
export const getAssetBalance = async (
  api: ApiPromise,
  assetID: number,
  accountAddress: string,
): Promise<AssetBalance> => {
  const assetRes = await api.query.assets.account<Option<AssetBalance>>(assetID, accountAddress);
  return assetRes.unwrapOrDefault();
};
