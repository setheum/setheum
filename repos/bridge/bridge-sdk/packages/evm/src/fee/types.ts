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

import type { FeeHandlerType } from '@buildwithsygma/core';
import type { ethers } from 'ethers';

import type { EvmFee } from '../types.js';

/**
 * Parameters that are required to
 * calculate fee for a Sygma transfer.
 */
export interface EvmFeeCalculationParams {
  provider: ethers.providers.Provider;
  sender: string;
  sourceSygmaId: number;
  destinationSygmaId: number;
  resourceSygmaId: string;
  feeHandlerAddress: string;
  feeHandlerType: FeeHandlerType;
  depositData?: string;
}

export interface EvmTransferFeeCalculationHandler {
  calculateFee(params: EvmFeeCalculationParams): Promise<EvmFee>;
  setNextHandler(handler: EvmTransferFeeCalculationHandler): EvmTransferFeeCalculationHandler;
}

export abstract class BaseEvmTransferFeeCalculator implements EvmTransferFeeCalculationHandler {
  nextHandler: EvmTransferFeeCalculationHandler | undefined;

  setNextHandler(handler: EvmTransferFeeCalculationHandler): EvmTransferFeeCalculationHandler {
    this.nextHandler = handler;
    return this.nextHandler;
  }

  /**
   * @category EvmFee
   *
   * Calculate transfer fee
   * @param {EvmFeeCalculationParams} params
   * @returns {Promise<EvmFee>}
   */
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  calculateFee(_params: EvmFeeCalculationParams): Promise<EvmFee> {
    throw new Error('Fee Calculation method not specified or undefined');
  }
}
