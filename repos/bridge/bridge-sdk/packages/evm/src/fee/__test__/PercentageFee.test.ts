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

/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-call */
import { FeeHandlerType } from '@buildwithsygma/core';
import type { JsonRpcBatchProvider } from '@ethersproject/providers';

import { PercentageFeeCalculator } from '../PercentageFee.js';
import type { EvmFeeCalculationParams } from '../types.js';

jest.mock('@buildwithsygma/sygma-contracts', () => {
  const { constants, BigNumber } = jest.requireActual('ethers');

  return {
    ...jest.requireActual('@buildwithsygma/sygma-contracts'),
    PercentageERC20FeeHandler__factory: {
      connect: jest.fn().mockReturnValue({
        calculateFee: () => Promise.resolve([constants.Zero]),
        _resourceIDToFeeBounds: jest.fn().mockResolvedValue({
          lowerBound: BigNumber.from('0'),
          upperBound: BigNumber.from('10000'),
        }),
        _domainResourceIDToFee: jest.fn().mockResolvedValue(BigNumber.from('100000')),
        HUNDRED_PERCENT: jest.fn().mockResolvedValue(BigNumber.from(10000)),
      }),
    },
  } as unknown;
});

describe('Percentage Fee Calculator', () => {
  const percentageFeeCalculator = new PercentageFeeCalculator();
  const mockedFeeCalculationParams = {
    provider: {} as JsonRpcBatchProvider,
    sender: '',
    sourceSygmaId: 1,
    destinationSygmaId: 2,
    resourceSygmaId: '0x0',
    feeHandlerAddress: '',
    feeHandlerType: FeeHandlerType.PERCENTAGE,
  } as EvmFeeCalculationParams;

  it('should calculate percentage fee', async () => {
    const fee = await percentageFeeCalculator.calculateFee(mockedFeeCalculationParams);
    expect(fee.fee).toEqual(0n);
  });
});
