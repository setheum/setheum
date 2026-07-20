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

import type { XcmMultiAssetIdType } from '@buildwithsygma/core';
import { FeeHandlerType } from '@buildwithsygma/core';
import type { ApiPromise } from '@polkadot/api';
import type { Enum, Option } from '@polkadot/types';

/**
 * Retrieves the fee handler for a given domainId and asset.
 *
 * @category Fee
 * @param {ApiPromise} api - The Substrate API instance.
 * @param {number} destinationDomainId - The ID of the domain.
 * @param {XcmMultiAssetIdType} xcmMultiAssetId - The XCM MultiAsset ID of the asset. {@link https://github.com/sygmaprotocol/sygma-substrate-pallets#multiasset | More details}
 * @returns {Promise} A promise that resolves to a FeeHandlerType object.
 * @throws {Error} Unable to retrieve fee from pallet
 */
export const getFeeHandler = async (
  api: ApiPromise,
  destinationDomainId: number,
  xcmMultiAssetId: XcmMultiAssetIdType,
): Promise<FeeHandlerType> => {
  const feeHandler = await api.query.sygmaFeeHandlerRouter.handlerType<Option<Enum>>([
    destinationDomainId,
    xcmMultiAssetId,
  ]);

  const feeHandlerName = feeHandler.unwrap().toString();

  switch (feeHandlerName) {
    case 'PercentageFeeHandler':
      return FeeHandlerType.PERCENTAGE;
    case 'BasicFeeHandler':
      return FeeHandlerType.BASIC;
    default:
      throw new Error('Invalid Fee Handler Type');
  }
};
