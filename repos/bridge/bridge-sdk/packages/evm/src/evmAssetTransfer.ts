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
import { isValidAddressForNetwork } from '@buildwithsygma/core';
import { Bridge__factory } from '@buildwithsygma/sygma-contracts';
import { Web3Provider } from '@ethersproject/providers';
import type { ethers, PayableOverrides } from 'ethers';
import { constants, utils } from 'ethers';

import { EvmTransfer } from './evmTransfer.js';
import type { EvmAssetTransferParams, EvmFee, TransactionRequest } from './types.js';
import { getTransactionOverrides } from './utils/depositFn.js';
import { createTransactionRequest } from './utils/transaction.js';

/**
 * Asset transfers in EVM
 * are of supported standards
 * like ERC20, ERC721 and ERC1155
 * which require set of approval and
 * transfer transactions
 * TODO: Add Support for all
 */
interface IAssetTransfer {
  getTransferTransaction(overrides?: ethers.Overrides): Promise<TransactionRequest>;
  getApprovalTransactions(overrides?: ethers.Overrides): Promise<Array<TransactionRequest>>;
}

/**
 *
 */
export abstract class AssetTransfer extends EvmTransfer implements IAssetTransfer {
  // amount in case if its a fungible transfer
  protected specifiedAmount?: bigint;
  protected adjustedAmount?: bigint;
  // tokenId in case if its a non fungible transfer
  protected tokenId?: string;
  // Recipient address, marked "!"
  // as it is initialized by a setter
  // in the constructor
  recipient!: string;

  // Recipient address
  get recipientAddress(): string {
    return this.recipient;
  }

  protected constructor(assetTransferParams: EvmAssetTransferParams, config: Config) {
    super(assetTransferParams, config);
    this.specifiedAmount = assetTransferParams.amount;
    this.adjustedAmount = assetTransferParams.amount;
    this.tokenId = assetTransferParams.tokenId;
    this.setRecipientAddress(assetTransferParams.recipientAddress);
  }

  /* eslint-disable @typescript-eslint/no-unused-vars */
  protected hasEnoughBalance(fee?: EvmFee): Promise<boolean> {
    throw new Error('Method not implemented.');
  }

  public getApprovalTransactions(): Promise<Array<TransactionRequest>> {
    throw new Error('Method not implemented.');
  }

  /**
   * Get transfer transaction
   * @returns {Promise<TransactionRequest>}
   */
  public async getTransferTransaction(overrides?: PayableOverrides): Promise<TransactionRequest> {
    const domainConfig = this.config.getDomainConfig(this.source);
    const provider = new Web3Provider(this.sourceNetworkProvider);
    const bridge = Bridge__factory.connect(domainConfig.bridge, provider);
    const fee = await this.getFee();

    const hasBalance = await this.hasEnoughBalance(fee);
    if (!hasBalance) throw new Error('Insufficient token balance');

    const transferTransaction = await bridge.populateTransaction.deposit(
      this.destinationDomain.id.toString(),
      this.resource.resourceId,
      this.getDepositData(),
      '0x',
      getTransactionOverrides(fee, overrides),
    );

    return createTransactionRequest(transferTransaction);
  }

  /**
   * Set recipient address
   * @param {string} address
   */
  public setRecipientAddress(address: string): void {
    if (isValidAddressForNetwork(this.environment, address, this.destination.type))
      this.recipient = address;
  }

  /**
   * Checks whether the
   * resource is registered
   * within Sygma protocol
   * @returns {Promise<boolean>}
   */
  public async isValidTransfer(): Promise<boolean> {
    const sourceDomainConfig = this.config.getDomainConfig(this.source);
    const web3Provider = new Web3Provider(this.sourceNetworkProvider);
    const bridge = Bridge__factory.connect(sourceDomainConfig.bridge, web3Provider);
    const { resourceId } = this.resource;
    const handlerAddress = await bridge._resourceIDToHandlerAddress(resourceId);
    return utils.isAddress(handlerAddress) && handlerAddress !== constants.AddressZero;
  }
}
