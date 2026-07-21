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

import { FeeHandlerType } from '@buildwithsygma/core';
import type { XcmMultiAssetIdType } from '@buildwithsygma/core/src';
import type { ApiPromise } from '@polkadot/api';
import type { Option, u128 } from '@polkadot/types';
import { BN } from '@polkadot/util';

import type { SubstrateFee } from '../types.js';
/**
 * Retrieves the basic fee for a given domainId and asset.
 *
 * @example
 * // Assuming the api instance is already connected and ready to use
 * const domainId = 1;
 * const xcmMultiAssetId = {...} // XCM MultiAsset ID of the asset
 * getBasicFee(api, domainId, xcmMultiAssetId)
 *   .catch((error) => {
 *     console.error('Error fetching basic fee:', error);
 *   });
 *
 * @category Fee
 * @param {ApiPromise} api - The Substrate API instance.
 * @param {number} destinationDomainId - The ID of the domain.
 * @param {XcmMultiAssetIdType} xcmMultiAssetId - The XCM MultiAsset ID of the asset. {@link https://github.com/sygmaprotocol/sygma-substrate-pallets#multiasset | More details}
 * @returns {Promise<SubstrateFee>} A promise that resolves to a SubstrateFee object.
 * @throws {Error} Unable to retrieve fee from pallet
 */
export const getBasicFee = async (
  api: ApiPromise,
  destinationDomainId: number,
  xcmMultiAssetId: XcmMultiAssetIdType,
): Promise<SubstrateFee> => {
  const rawFee = await api.query.sygmaBasicFeeHandler.assetFees<Option<u128>>([
    destinationDomainId,
    xcmMultiAssetId,
  ]);

  if (rawFee.isNone) {
    throw new Error('Error retrieving fee.');
  }

  return {
    fee: new BN(rawFee.unwrap()),
    type: FeeHandlerType.BASIC,
  };
};
