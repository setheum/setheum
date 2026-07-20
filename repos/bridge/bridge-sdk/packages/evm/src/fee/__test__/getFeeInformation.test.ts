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

import type { Config } from '@buildwithsygma/core';
import { FeeHandlerType } from '@buildwithsygma/core';
import { FeeHandlerRouter__factory } from '@buildwithsygma/sygma-contracts';
import type { JsonRpcProvider } from '@ethersproject/providers';

import { getFeeInformation } from '../getFeeInformation.js';

jest.mock(
  '@buildwithsygma/sygma-contracts',
  () =>
    ({
      ...jest.requireActual('@buildwithsygma/sygma-contracts'),
      FeeHandlerRouter__factory: {
        connect: jest.fn().mockReturnValue({
          _domainResourceIDToFeeHandlerAddress: jest
            .fn()
            .mockResolvedValue('0x98729c03c4D5e820F5e8c45558ae07aE63F97461'),
        }),
      },
      BasicFeeHandler__factory: {
        connect: jest.fn().mockReturnValue({
          feeHandlerType: () => 'basic',
        }),
      },
      Bridge__factory: {
        connect: jest.fn().mockReturnValue({
          _feeHandler: jest.fn().mockResolvedValue('0x0000000000000000000000000000000000000000'),
        }),
      },
    }) as unknown,
);

describe('getFeeInformation()', () => {
  const feeInfoParams = {
    config: {
      findDomainConfigBySygmaId: jest.fn().mockReturnValue({
        feeRouter: '',
        feeHandlers: [
          {
            address: '0x98729c03c4D5e820F5e8c45558ae07aE63F97461',
            type: FeeHandlerType.BASIC,
          },
        ],
      }),
    },
    sourceProvider: {},
    sygmaSourceId: 1,
    sygmaDestinationDomainId: 1,
    sygmaResourceId: '0x',
  };

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('should provide fee handler configuration', async () => {
    const feeInformation = await getFeeInformation(
      feeInfoParams.config as unknown as Config,
      feeInfoParams.sourceProvider as unknown as JsonRpcProvider,
      feeInfoParams.sygmaSourceId,
      feeInfoParams.sygmaDestinationDomainId,
      feeInfoParams.sygmaResourceId,
    );

    expect(feeInformation.feeHandlerType).toEqual(FeeHandlerType.BASIC);
  });

  it('should throw error when fee handler is not configured', async () => {
    (FeeHandlerRouter__factory.connect as jest.Mock).mockImplementation(() => ({
      _domainResourceIDToFeeHandlerAddress: jest
        .fn()
        .mockResolvedValue('0x0000000000000000000000000000000000000000'),
    }));

    await expect(
      async () =>
        await getFeeInformation(
          feeInfoParams.config as unknown as Config,
          feeInfoParams.sourceProvider as unknown as JsonRpcProvider,
          feeInfoParams.sygmaSourceId,
          feeInfoParams.sygmaDestinationDomainId,
          feeInfoParams.sygmaResourceId,
        ),
    ).rejects.toThrow('Fee Handler not found for Resource ID 0x to Domain 1');
  });
});
