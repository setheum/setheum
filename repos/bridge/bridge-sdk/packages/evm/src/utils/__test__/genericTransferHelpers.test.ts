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

import { Network } from '@buildwithsygma/core';

import { createGenericCallDepositData } from '../genericTransferHelpers.js';

const CONTRACT_ABI = [
  {
    inputs: [
      {
        internalType: 'address',
        name: '_depositer',
        type: 'address',
      },
      {
        internalType: 'address',
        name: '_index',
        type: 'address',
      },
      {
        internalType: 'uint256',
        name: '_value',
        type: 'uint256',
      },
    ],
    name: 'store',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'address',
        name: '_index',
        type: 'address',
      },
    ],
    name: 'retrieve',
    outputs: [
      {
        internalType: 'uint256',
        name: '',
        type: 'uint256',
      },
    ],
    stateMutability: 'view',
    type: 'function',
  },
] as const;

describe('createGenericCallDepositData', () => {
  it('should create correct generic call deposit data', () => {
    const depositor = '0x98729c03c4D5e820F5e8c45558ae07aE63F97461' as const;

    const genericCallParams = {
      abi: CONTRACT_ABI,
      functionName: 'store',
      functionParams: [depositor, depositor, BigInt(42069)] as const,
      contractAddress: '0x4bE595ab5A070663B314970Fc10C049BBA0ad489',
      destination: {
        name: 'EVM',
        type: Network.EVM,
        caipId: '11',
        chainId: 1,
        id: 1,
      },
      maxFee: BigInt(3000000),
      depositor: depositor,
    };

    const expectedDepositData =
      '0x00000000000000000000000000000000000000000000000000000000002dc6c00004ba154fea144be595ab5a070663b314970fc10c049bba0ad4891498729c03c4d5e820f5e8c45558ae07ae63f9746100000000000000000000000098729c03c4d5e820f5e8c45558ae07ae63f97461000000000000000000000000000000000000000000000000000000000000a455';

    const depositData = createGenericCallDepositData<typeof CONTRACT_ABI, 'store'>(
      genericCallParams,
    );
    expect(depositData.toLowerCase()).toEqual(expectedDepositData.toLowerCase());
  });
});
