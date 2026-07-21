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
import { BasicFeeHandler__factory } from '@buildwithsygma/sygma-contracts';
import { ethers } from 'ethers';

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
 * basic calculation.
 */
export class BasicFeeCalculator extends BaseEvmTransferFeeCalculator {
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
    } = params;

    if (feeHandlerType === FeeHandlerType.BASIC) {
      const BasicFeeHandlerInstance = BasicFeeHandler__factory.connect(feeHandlerAddress, provider);

      const calculatedFee = await BasicFeeHandlerInstance.calculateFee(
        sender,
        sourceSygmaId,
        destinationSygmaId,
        resourceSygmaId,
        ethers.utils.formatBytes32String(''),
        ethers.utils.formatBytes32String(''),
      );

      const [fee] = calculatedFee;

      return {
        fee: fee.toBigInt(),
        type: FeeHandlerType.BASIC,
        handlerAddress: feeHandlerAddress,
      };
    }

    if (this.nextHandler) {
      return this.nextHandler.calculateFee(params);
    }

    return super.calculateFee(params);
  }
}
