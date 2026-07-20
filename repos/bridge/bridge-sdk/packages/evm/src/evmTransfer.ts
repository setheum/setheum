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

import type { Config, HexString, Eip1193Provider } from '@buildwithsygma/core';
import { BaseTransfer } from '@buildwithsygma/core';
import { providers } from 'ethers';

import { TwapFeeCalculator } from './fee/TwapFee.js';
import { getFeeInformation, BasicFeeCalculator, PercentageFeeCalculator } from './fee/index.js';
import type { EvmFee, EvmTransferParams } from './types.js';

/**
 * @internal
 * @class EvmTransfer
 *
 * @abstract
 * Base EVM transfer class
 * housing common functionality
 */
export abstract class EvmTransfer extends BaseTransfer {
  protected provider: Eip1193Provider;

  get sourceNetworkProvider(): Eip1193Provider {
    return this.provider;
  }

  protected constructor(params: EvmTransferParams, config: Config) {
    super(params, config);
    this.provider = params.sourceNetworkProvider;
  }

  /**
   * Deposit Data is required
   * by the sygma protocol to process
   * transfer types
   * @returns {string}
   */
  protected getDepositData(): string {
    throw new Error('Method not implemented.');
  }

  /**
   * Returns fee based on transfer amount.
   * @returns {Promise<EvmFee>}
   */
  public async getFee(): Promise<EvmFee> {
    const provider = new providers.Web3Provider(this.sourceNetworkProvider);

    const { feeHandlerAddress, feeHandlerType } = await getFeeInformation(
      this.config,
      provider,
      this.source.id,
      this.destination.id,
      this.resource.resourceId,
    );

    const basicFeeCalculator = new BasicFeeCalculator();
    const percentageFeeCalculator = new PercentageFeeCalculator();
    const twapFeeCalculator = new TwapFeeCalculator();

    basicFeeCalculator.setNextHandler(percentageFeeCalculator).setNextHandler(twapFeeCalculator);

    return await basicFeeCalculator.calculateFee({
      provider,
      sender: this.sourceAddress,
      sourceSygmaId: this.source.id,
      destinationSygmaId: this.destination.id,
      resourceSygmaId: this.resource.resourceId,
      feeHandlerAddress,
      feeHandlerType,
      depositData: this.getDepositData(),
    });
  }

  public setSourceAddress(address: HexString): void {
    this.sourceAddress = address;
  }
}
