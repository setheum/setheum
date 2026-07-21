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
import type { JsonRpcBatchProvider } from '@ethersproject/providers';
import { constants } from 'ethers';

import { BasicFeeCalculator } from '../BasicFee.js';
import type { EvmFeeCalculationParams } from '../types.js';

jest.mock(
  '@buildwithsygma/sygma-contracts',
  () =>
    ({
      ...jest.requireActual('@buildwithsygma/sygma-contracts'),
      BasicFeeHandler__factory: {
        connect: jest.fn().mockReturnValue({
          calculateFee: () => Promise.resolve([constants.Zero]),
        }),
      },
    }) as unknown,
);

describe('Basic Fee Calculator', () => {
  const basicFeeCalculator = new BasicFeeCalculator();
  const mockedFeeCalculationParams = {
    provider: {} as JsonRpcBatchProvider,
    sender: '',
    sourceSygmaId: 1,
    destinationSygmaId: 2,
    resourceSygmaId: '0x0',
    feeHandlerAddress: '',
    feeHandlerType: FeeHandlerType.BASIC,
  } as EvmFeeCalculationParams;

  it('should return calculated fee', async () => {
    const fee = await basicFeeCalculator.calculateFee(mockedFeeCalculationParams);
    expect(fee.fee).toEqual(0n);
  });

  it('should throw error if fee handler type is not BASIC', async () => {
    mockedFeeCalculationParams.feeHandlerType = FeeHandlerType.PERCENTAGE;

    await expect(basicFeeCalculator.calculateFee(mockedFeeCalculationParams)).rejects.toThrow(
      'Fee Calculation method not specified or undefined',
    );
  });
});
