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
import type { Bridge } from '@buildwithsygma/sygma-contracts';

import { getEvmErc20Balance, getEvmHandlerBalance } from '../balances.js';

jest.mock('@buildwithsygma/sygma-contracts', () => {
  return {
    ...jest.requireActual('@buildwithsygma/sygma-contracts'),
    ERC20__factory: {
      connect: jest.fn().mockReturnValue({
        balanceOf: jest.fn().mockResolvedValue({
          toBigInt: () => 0n,
          toString: () => '0',
        }),
      }) as unknown as Bridge,
    },
  } as unknown;
});

jest.mock('@ethersproject/providers', () => {
  return {
    ...jest.requireActual('@ethersproject/providers'),
    Web3Provider: jest.fn().mockReturnValue({
      getBalance: jest.fn().mockResolvedValue({
        toBigInt: () => 0n,
        toString: () => '0',
      }),
    }),
  } as unknown;
});

// Define a test suite
describe('Balances', () => {
  const provider = jest.fn() as unknown as Eip1193Provider;

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('Should provide erc20 balance', async function () {
    const balance = await getEvmErc20Balance(provider, '', '');
    expect(balance).toEqual(0n);
  });

  it('Should provide native handler balance', async function () {
    const nativeHandlerBalance = await getEvmHandlerBalance(
      provider,
      { native: true } as unknown as EvmResource,
      '',
    );
    expect(nativeHandlerBalance).toEqual(0n);
  });

  it('Should provide erc20 handler balance', async function () {
    const nativeHandlerBalance = await getEvmHandlerBalance(
      provider,
      { native: false } as unknown as EvmResource,
      '',
    );
    expect(nativeHandlerBalance).toEqual(0n);
  });
});
