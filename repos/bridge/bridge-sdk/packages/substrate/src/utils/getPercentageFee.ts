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
import type { ApiPromise } from '@polkadot/api';
import type { Option, Tuple } from '@polkadot/types';
import { BN } from '@polkadot/util';

import type { Fungible, SubstrateFee, Transfer } from '../types.js';

/**
 * Retrieves the basic fee for a given domainId and asset.
 *
 * @example
 * // Assuming the api instance is already connected and ready to use
 * const domainId = 1;
 * const xcmMultiAssetId = {...} // XCM MultiAsset ID of the asset
 * getPercentageFee(api, transfer)
 *   .catch((error) => {
 *     console.error('Error fetching basic fee:', error);
 *   });
 *
 * @category Fee
 * @param {ApiPromise} api - The Substrate API instance.
 * @param {Transfer<Fungible>} transfer
 * @returns {Promise<SubstrateFee>} A promise that resolves to a SubstrateFee object.
 * @throws {Error} Unable to retrieve fee from pallet
 */
export const getPercentageFee = async (
  api: ApiPromise,
  transfer: Transfer<Fungible>,
): Promise<SubstrateFee> => {
  const assetId = transfer.resource.xcmMultiAssetId;

  const feeStructure = await api.query.sygmaPercentageFeeHandler.assetFeeRate<Option<Tuple>>([
    transfer.to.id,
    assetId,
  ]);

  if (feeStructure.isNone) {
    throw new Error('Error retrieving fee.');
  }

  const [feeRate, min, max] = feeStructure
    .unwrap()
    .toArray()
    .map(val => new BN(val.toString()));

  const calculatedFee = new BN(transfer.details.amount).mul(feeRate).div(new BN(10000));

  let fee: BN;
  if (calculatedFee.lt(min)) {
    fee = min;
  } else if (calculatedFee.gt(max)) {
    fee = max;
  } else {
    fee = calculatedFee;
  }

  return {
    fee: fee,
    type: FeeHandlerType.PERCENTAGE,
  };
};
