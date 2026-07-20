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

import { Network, ResourceType } from '@buildwithsygma/core';
import type { createFungibleAssetTransfer } from '@buildwithsygma/evm';

import { hasEnoughLiquidity } from '../liquidity.js';

// eslint-disable-next-line @typescript-eslint/no-unsafe-return
jest.mock('web3-providers-http', () => ({
  ...jest.requireActual('web3-providers-http'),
  HttpProvider: jest.fn(),
}));
// eslint-disable-next-line @typescript-eslint/no-unsafe-return
jest.mock('@buildwithsygma/evm', () => ({
  ...jest.requireActual('@buildwithsygma/evm'),
  getEvmHandlerBalance: jest.fn().mockResolvedValue(BigInt(5)),
}));

jest.mock('../substrate/balances.js', () => ({
  getSubstrateHandlerBalance: jest.fn().mockResolvedValue(BigInt(5)),
}));

const mockedHandler = {
  type: ResourceType.FUNGIBLE,
  address: '',
};

const mockedResource = {
  resourceId: '0x00',
  caip19: 'caipId',
  type: ResourceType.FUNGIBLE,
  address: '0x123',
};

const mockedDestination = {
  id: 1,
  caipId: 'caipId',
  chainId: 1,
  name: 'Chain',
  type: Network.EVM,
};

const mockedTransferEVM = {
  transferAmount: 0n,
  resource: mockedResource,
  config: {
    findDomainConfig: jest.fn(),
  },
};

const mockedTransferSubstrate = {
  transferAmount: 0n,
  resource: mockedResource,
  config: {
    findDomainConfig: jest.fn(),
  },
};

const destinationProviderUrl = 'mockedProviderUrl';

describe('hasEnoughLiquidity - EVM', () => {
  beforeAll(() => {
    Object.assign(mockedTransferEVM, { destination: mockedDestination });

    mockedTransferEVM.config.findDomainConfig.mockReturnValue({
      handlers: [mockedHandler],
      resources: [mockedResource],
    });
  });

  it('should return true if there is enough liquidity', async () => {
    mockedTransferEVM.transferAmount = BigInt(1);

    const isEnough = await hasEnoughLiquidity(
      mockedTransferEVM as unknown as Awaited<ReturnType<typeof createFungibleAssetTransfer>>,
      destinationProviderUrl,
    );

    expect(isEnough).toEqual(true);
  });

  it('should return false if there isnt enough liquidity', async () => {
    mockedTransferEVM.transferAmount = BigInt(10);

    const isEnough = await hasEnoughLiquidity(
      mockedTransferEVM as unknown as Awaited<ReturnType<typeof createFungibleAssetTransfer>>,
      destinationProviderUrl,
    );

    expect(isEnough).toEqual(false);
  });
  it('should throw error if handler is not found', async () => {
    const mockedTransferClone = Object.assign({}, mockedTransferEVM);
    mockedTransferClone.config.findDomainConfig = jest
      .fn()
      .mockReturnValue({ handlers: [], resources: [mockedResource] });

    await expect(
      hasEnoughLiquidity(
        mockedTransferClone as unknown as Awaited<ReturnType<typeof createFungibleAssetTransfer>>,
        destinationProviderUrl,
      ),
    ).rejects.toThrow('Handler not found or unregistered for resource.');
  });
  it('should throw error if resource is not found', async () => {
    const mockedTransferClone = Object.assign({}, mockedTransferEVM);
    mockedTransferClone.config.findDomainConfig = jest
      .fn()
      .mockReturnValue({ handlers: [mockedHandler], resources: [] });

    await expect(
      hasEnoughLiquidity(
        mockedTransferClone as unknown as Awaited<ReturnType<typeof createFungibleAssetTransfer>>,
        destinationProviderUrl,
      ),
    ).rejects.toThrow('Resource not found or unregistered.');
  });
});

describe('hasEnoughLiquidity - substrate', () => {
  beforeAll(() => {
    Object.assign(mockedTransferSubstrate, {
      destination: { ...mockedDestination, type: Network.SUBSTRATE },
    });

    mockedTransferSubstrate.config.findDomainConfig.mockReturnValue({
      handlers: [mockedHandler],
      resources: [mockedResource],
    });
  });

  it('should return true if there is enough liquidity', async () => {
    mockedTransferSubstrate.transferAmount = BigInt(5);

    const isEnough = await hasEnoughLiquidity(
      mockedTransferSubstrate as unknown as Awaited<ReturnType<typeof createFungibleAssetTransfer>>,
      destinationProviderUrl,
    );

    expect(isEnough).toEqual(true);
  });
  it('should return false if there isnt enough liquidity', async () => {
    mockedTransferSubstrate.transferAmount = BigInt(10);

    const isEnough = await hasEnoughLiquidity(
      mockedTransferSubstrate as unknown as Awaited<ReturnType<typeof createFungibleAssetTransfer>>,
      destinationProviderUrl,
    );

    expect(isEnough).toEqual(false);
  });
});

describe('hasEnoughLiquidity - error', () => {
  beforeAll(() => {
    Object.assign(mockedTransferEVM, {
      destination: { ...mockedDestination, type: 'foo' as unknown as Network },
    });

    mockedTransferEVM.config.findDomainConfig.mockReturnValue({
      handlers: [mockedHandler],
      resources: [mockedResource],
    });
  });

  it('should return false if network type is not supported', async () => {
    const isEnough = await hasEnoughLiquidity(
      mockedTransferSubstrate as unknown as Awaited<ReturnType<typeof createFungibleAssetTransfer>>,
      destinationProviderUrl,
    );

    expect(isEnough).toEqual(false);
  });
});
