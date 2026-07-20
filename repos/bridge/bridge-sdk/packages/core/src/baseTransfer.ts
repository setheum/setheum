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

import type { Config } from './config/config.js';
import { Environment } from './types.js';
import type {
  Domainlike,
  EvmResource,
  Domain,
  SubstrateResource,
  BitcoinResource,
} from './types.js';

export interface BaseTransferParams {
  source: Domainlike;
  destination: Domainlike;
  resource: string | EvmResource | SubstrateResource | BitcoinResource;
  sourceAddress: string;
  environment?: Environment;
}

export abstract class BaseTransfer {
  protected destinationDomain: Domain;
  protected sourceDomain: Domain;
  protected transferResource: EvmResource | SubstrateResource | BitcoinResource;
  protected sygmaConfiguration: Config;
  protected sourceAddress: string;
  protected environment: Environment;

  public get source(): Domain {
    return this.sourceDomain;
  }

  public get destination(): Domain {
    return this.destinationDomain;
  }

  public get resource(): EvmResource | SubstrateResource | BitcoinResource {
    return this.transferResource;
  }

  public get config(): Config {
    return this.sygmaConfiguration;
  }

  private findResource(
    resource: string | EvmResource | SubstrateResource | BitcoinResource,
  ): EvmResource | SubstrateResource | BitcoinResource | undefined {
    return this.sygmaConfiguration.getResources(this.source).find(_resource => {
      return typeof resource === 'string'
        ? resource === _resource.resourceId
        : resource.resourceId === _resource.resourceId;
    });
  }

  protected constructor(transfer: BaseTransferParams, config: Config) {
    this.sygmaConfiguration = config;
    this.sourceAddress = transfer.sourceAddress;
    this.sourceDomain = config.getDomain(transfer.source);
    this.destinationDomain = config.getDomain(transfer.destination);
    this.environment = transfer.environment ?? Environment.MAINNET;
    const resource = this.findResource(transfer.resource);

    if (resource) {
      this.transferResource = resource;
    } else {
      throw new Error('Resource not found.');
    }
  }
  /**
   * Method that checks whether the transfer
   * is valid and route has been registered on
   * the bridge
   * @returns {boolean}
   */
  // eslint-disable-next-line @typescript-eslint/require-await
  async isValidTransfer(): Promise<boolean> {
    throw new Error('Method not implemented.');
  }
  /**
   * Set resource to be transferred
   * @param {EvmResource | SubstrateResource | BitcoinResource} resource
   * @returns {BaseTransfer}
   */
  setResource(resource: EvmResource | SubstrateResource | BitcoinResource): void {
    this.transferResource = resource;
  }
  /**
   *
   * @param destination
   * @returns
   */
  setDestinationDomain(destination: Domainlike): void {
    this.destinationDomain = this.config.getDomain(destination);
  }
}
