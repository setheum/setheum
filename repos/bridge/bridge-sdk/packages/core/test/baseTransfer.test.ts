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

import { enableFetchMocks } from 'jest-fetch-mock';
import { BaseTransfer, BaseTransferParams } from '../src/baseTransfer.js';
import { ConfigUrl } from '../src/constants.js';
import { Config, Environment, EvmResource, ResourceType } from '../src/index.js';
import { mockedDevnetConfig } from './constants.js';

enableFetchMocks();

const TRANSFER_PARAMS: BaseTransferParams = {
  source: 111,
  destination: 111,
  resource: '0x0000000000000000000000000000000000000000000000000000000000000000',
  sourceAddress: '0x00',
};

class Transfer extends BaseTransfer {
  constructor(params: BaseTransferParams, config: Config) {
    super(params, config);
  }
}

describe('BaseTransfer', () => {
  let config: Config;

  beforeAll(async () => {
    jest.clearAllMocks();
    fetchMock.resetMocks();
    fetchMock.doMock();
    fetchMock.mockIf(ConfigUrl.DEVNET.toString(), JSON.stringify(mockedDevnetConfig));
    config = new Config();
    await config.init(Environment.DEVNET);
  });

  it('should be able to instantiate a transfer object', async () => {
    const transfer = new Transfer(TRANSFER_PARAMS, config);
    expect(transfer).toBeInstanceOf(Transfer);
  });

  it('should not be able to instantiate a transfer object with an invalid domain', async () => {
    const config = new Config();
    await config.init(Environment.DEVNET);

    expect(() => new Transfer({ ...TRANSFER_PARAMS, destination: 54 }, config)).toThrow(
      'Domain configuration not found.',
    );
  });

  it('should be able to set resource', async () => {
    const config = new Config();
    await config.init(Environment.DEVNET);
    const transfer = new Transfer(TRANSFER_PARAMS, config);

    const resource: EvmResource = {
      resourceId: '0x00',
      caip19: 'caip:id',
      type: ResourceType.FUNGIBLE,
      address: '0x00',
    };

    transfer.setResource(resource);
    expect(transfer.resource.caip19).toEqual(resource.caip19);
  });

  it('should be able to set destination domain', async () => {
    const config = new Config();
    await config.init(Environment.DEVNET);
    const transfer = new Transfer(TRANSFER_PARAMS, config);

    transfer.setDestinationDomain('ethereum:1');
    expect(transfer.destination).toBeTruthy();
  });
});
