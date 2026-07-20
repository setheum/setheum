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

import type { Eip1193Provider, EvmResource } from '@buildwithsygma/core';
import { ERC20__factory } from '@buildwithsygma/sygma-contracts';
import { Web3Provider } from '@ethersproject/providers';

/**
 * Get liquidity of resource handler on destination domain
 * @param {Eip1193Provider} provider
 * @param {EvmResource} resource sygma transferable resource
 * @param {string} handlerAddress address of resource handler
 * @returns {Promise<bigint>} handler balance
 */
export const getEvmHandlerBalance = async (
  provider: Eip1193Provider,
  resource: EvmResource,
  handlerAddress: string,
): Promise<bigint> => {
  const web3Provider = new Web3Provider(provider);
  if (resource.native) {
    return (await web3Provider.getBalance(handlerAddress)).toBigInt();
  } else {
    const tokenAddress = resource.address;
    return await getEvmErc20Balance(provider, tokenAddress, handlerAddress);
  }
};

/**
 * Fetch ERC20 token balance of an address
 * @param {Eip1193Provider} provider Network provider
 * @param {string} tokenAddress ERC20 token address
 * @param {string} address EVM address to query
 * @returns {Promise<bigint>} balance
 */
export const getEvmErc20Balance = async (
  provider: Eip1193Provider,
  tokenAddress: string,
  address: string,
): Promise<bigint> => {
  const web3Provider = new Web3Provider(provider);
  const erc20Contract = ERC20__factory.connect(tokenAddress, web3Provider);
  return BigInt((await erc20Contract.balanceOf(address)).toString());
};
