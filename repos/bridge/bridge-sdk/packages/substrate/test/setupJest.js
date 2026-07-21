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

require('jest-fetch-mock').enableMocks();
require('dotenv').config({ path: '.env.test' });

jest.mock('@polkadot/api', () => {
  const originalModule = jest.requireActual('@polkadot/api');

  const createMockApi = ({
    balance = {
      free: '500',
      reserved: '5000',
      miscFrozen: '5000',
      feeFrozen: '5000',
    },
    chainProperties = {
      tokenDecimals: ['12'],
      tokenSymbol: ['DOT'],
    },
  } = {}) => {
    const mockBalance = {
      data: balance,
    };

    return {
      query: {
        system: {
          account: jest.fn().mockResolvedValue(mockBalance),
        },
      },
      registry: {
        getChainProperties: jest.fn().mockReturnValue(chainProperties),
      },
    };
  };

  return {
    ...originalModule,
    WsProvider: jest.fn(() => ({
      connect: jest.fn(),
      disconnect: jest.fn(),
      isConnected: jest.fn().mockReturnValue(true),
      send: jest.fn(),
      on: jest.fn(),
      off: jest.fn(),
    })),
    ApiPromise: {
      create: jest.fn().mockImplementation(config => {
        return Promise.resolve(createMockApi(config));
      }),
    },
  };
});
