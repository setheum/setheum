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

import type { SubstrateResource } from '@buildwithsygma/core';
import { getAssetBalance, getNativeTokenBalance } from '@buildwithsygma/substrate';
import { ApiPromise } from '@polkadot/api';
import { WsProvider } from '@polkadot/rpc-provider';

export const getSubstrateHandlerBalance = async (
  destinationProviderUrl: string,
  resource: SubstrateResource,
  handlerAddress: string,
): Promise<bigint> => {
  const wsProvider = new WsProvider(destinationProviderUrl);
  const apiPromise = new ApiPromise({ provider: wsProvider });
  if (resource.native) {
    const accountInfo = await getNativeTokenBalance(apiPromise, handlerAddress);
    return BigInt(accountInfo.free.toString());
  } else {
    const assetBalance = await getAssetBalance(apiPromise, resource.assetID ?? 0, handlerAddress);
    return BigInt(assetBalance.balance.toString());
  }
};
