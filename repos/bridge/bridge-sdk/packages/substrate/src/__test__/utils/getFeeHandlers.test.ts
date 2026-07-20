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

import type { XcmMultiAssetIdType } from '@buildwithsygma/core';
import { FeeHandlerType } from '@buildwithsygma/core';
import type { ApiPromise } from '@polkadot/api';
import { Option, Enum } from '@polkadot/types';
import { TypeRegistry } from '@polkadot/types/create';

import { getFeeHandler } from '../../utils/getFeeHandlers.js';

const registry = new TypeRegistry();

describe('Substrate - getFeeHandler', () => {
  const FEE_HANDLER_NAMES = Enum.with([
    'PercentageFeeHandler',
    'BasicFeeHandler',
    'DynamicFeeHandler',
  ]);
  const validXcmMultiAssetId: XcmMultiAssetIdType = {
    concrete: {
      parents: 1,
      interior: {
        x3: [
          {
            parachain: 2004,
          },
          {
            generalKey: [5, '0x12345'],
          },
          {
            generalKey: [4, '0x1234'],
          },
        ],
      },
    },
  };

  it('should return PERCENTAGE fee handler type', async () => {
    const mockFeeHandlerType = new Option(registry, FEE_HANDLER_NAMES, 'PercentageFeeHandler');
    const api: ApiPromise = {
      query: {
        sygmaFeeHandlerRouter: {
          handlerType: jest.fn().mockResolvedValue(mockFeeHandlerType),
        },
      },
    } as unknown as ApiPromise;

    const destinationDomainId = 1;

    const feeHandlerType = await getFeeHandler(api, destinationDomainId, validXcmMultiAssetId);

    expect(feeHandlerType).toBe(FeeHandlerType.PERCENTAGE);
  });

  it('should return BASIC fee handler type', async () => {
    const mockFeeHandlerType = new Option(registry, FEE_HANDLER_NAMES, 'BasicFeeHandler');

    const api: ApiPromise = {
      query: {
        sygmaFeeHandlerRouter: {
          handlerType: jest.fn().mockResolvedValue(mockFeeHandlerType),
        },
      },
    } as unknown as ApiPromise;

    const destinationDomainId = 1;

    const feeHandlerType = await getFeeHandler(api, destinationDomainId, validXcmMultiAssetId);

    expect(feeHandlerType).toBe(FeeHandlerType.BASIC);
  });

  it('should throw an error for an invalid fee handler type', async () => {
    const mockFeeHandlerType = new Option(
      registry,
      Enum.with(['InvalidFeeHandler']),
      'InvalidFeeHandler',
    );

    const api: ApiPromise = {
      query: {
        sygmaFeeHandlerRouter: {
          handlerType: jest.fn().mockResolvedValue(mockFeeHandlerType),
        },
      },
    } as unknown as ApiPromise;

    const destinationDomainId = 1;

    await expect(getFeeHandler(api, destinationDomainId, validXcmMultiAssetId)).rejects.toThrow(
      'Invalid Fee Handler Type',
    );
  });
});
