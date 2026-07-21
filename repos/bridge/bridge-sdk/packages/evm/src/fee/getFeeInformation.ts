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

import type { Config, EthereumConfig, FeeHandlerType } from '@buildwithsygma/core';
import {
  BasicFeeHandler__factory,
  Bridge__factory,
  FeeHandlerRouter__factory,
} from '@buildwithsygma/sygma-contracts';
import { utils, ethers } from 'ethers';

import { UnregisteredFeeHandlerError } from '../errors.js';
/**
 * @internal
 * @category EVM Fee
 *
 *
 * Retrieves fee information
 * configured on chain for a specific route
 * Throws error if feeHandler is not registered
 * @param {Config} config
 * @param {ethers.providers.BaseProvider} sourceProvider
 * @param {number} sygmaSourceId
 * @param {number} sygmaDestinationDomainId
 * @param {string} sygmaResourceId
 * @returns {Promise<{feeHandlerAddress: string; feeHandlerType: FeeHandlerType;}>}
 */
export async function getFeeInformation(
  config: Config,
  sourceProvider: ethers.providers.BaseProvider,
  sygmaSourceId: number,
  sygmaDestinationDomainId: number,
  sygmaResourceId: string,
): Promise<{
  feeHandlerAddress: string;
  feeHandlerType: FeeHandlerType;
}> {
  const domainConfig = config.findDomainConfigBySygmaId(sygmaSourceId) as EthereumConfig;
  const bridgeInstance = Bridge__factory.connect(domainConfig.bridge, sourceProvider);
  const feeRouterAddress = await bridgeInstance._feeHandler();

  const feeRouter = FeeHandlerRouter__factory.connect(feeRouterAddress, sourceProvider);
  const feeHandlerAddress = await feeRouter._domainResourceIDToFeeHandlerAddress(
    sygmaDestinationDomainId,
    sygmaResourceId,
  );

  if (!utils.isAddress(feeHandlerAddress) || feeHandlerAddress === ethers.constants.AddressZero) {
    throw new UnregisteredFeeHandlerError(sygmaDestinationDomainId, sygmaResourceId);
  }

  const FeeHandler = BasicFeeHandler__factory.connect(feeHandlerAddress, sourceProvider);
  const feeHandlerType = (await FeeHandler.feeHandlerType()) as unknown as FeeHandlerType;
  return { feeHandlerAddress, feeHandlerType };
}
