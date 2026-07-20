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
import { PercentageERC20FeeHandler__factory } from '@buildwithsygma/sygma-contracts';
import { utils } from 'ethers';

import type { EvmFee } from '../types.js';

import type { EvmFeeCalculationParams } from './types.js';
import { BaseEvmTransferFeeCalculator } from './types.js';

/**
 * @internal
 * @category EVM Fee
 *
 *
 * Wrapper class to calculate
 * fee for a route that uses
 * percentage fee calculation.
 */
export class PercentageFeeCalculator extends BaseEvmTransferFeeCalculator {
  constructor() {
    super();
  }
  /**
   * @category EvmFee
   *
   * Calculate transfer fee
   * @param {EvmFeeCalculationParams} params
   * @returns {Promise<EvmFee>}
   */
  async calculateFee(params: EvmFeeCalculationParams): Promise<EvmFee> {
    const {
      feeHandlerAddress,
      feeHandlerType,
      sender,
      sourceSygmaId,
      destinationSygmaId,
      resourceSygmaId,
      provider,
      depositData,
    } = params;

    if (feeHandlerType === FeeHandlerType.PERCENTAGE) {
      const percentageFeeHandlerContract = PercentageERC20FeeHandler__factory.connect(
        feeHandlerAddress,
        provider,
      );

      const calculatedFee = await percentageFeeHandlerContract.calculateFee(
        sender,
        sourceSygmaId,
        destinationSygmaId,
        resourceSygmaId,
        depositData ?? utils.formatBytes32String(''),
        utils.formatBytes32String(''),
      );

      const feeBounds = await percentageFeeHandlerContract._resourceIDToFeeBounds(resourceSygmaId);

      const feePercentage = (
        await percentageFeeHandlerContract._domainResourceIDToFee(
          destinationSygmaId,
          resourceSygmaId,
        )
      ).toNumber();

      const HUNDRED_PERCENT = await percentageFeeHandlerContract.HUNDRED_PERCENT();
      const percentage = feePercentage / HUNDRED_PERCENT;
      const [fee] = calculatedFee;

      return {
        fee: fee.toBigInt(),
        percentage,
        type: FeeHandlerType.PERCENTAGE,
        handlerAddress: feeHandlerAddress,
        minFee: feeBounds.lowerBound.toBigInt(),
        maxFee: feeBounds.upperBound.toBigInt(),
      };
    }

    if (this.nextHandler) {
      return this.nextHandler.calculateFee(params);
    }

    return super.calculateFee(params);
  }
}
