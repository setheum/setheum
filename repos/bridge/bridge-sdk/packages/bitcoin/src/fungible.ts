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

import { BaseTransfer, Config, Environment } from '@buildwithsygma/core';
import type { Config as TConfig, BitcoinResource } from '@buildwithsygma/core/types';
import type { networks } from 'bitcoinjs-lib';

import type {
  BitcoinTransferParams,
  BitcoinTransaction,
  TypeOfAddress,
  UTXOData,
} from './types.js';
import { getPsbt } from './utils/index.js';

export async function createBitcoinFungibleTransfer(
  params: BitcoinTransferParams,
): Promise<BitcoinFungibleTransfer> {
  const config = new Config();
  await config.init(params.environment || Environment.MAINNET);
  return new BitcoinFungibleTransfer(params, config);
}

class BitcoinFungibleTransfer extends BaseTransfer {
  protected publicKey: Buffer;
  protected typeOfAddress: TypeOfAddress;
  protected network: networks.Network;
  protected changeAddress?: string;
  protected feeRate: bigint;
  protected utxoData: UTXOData[];
  protected size: bigint;
  protected destinationAddress: string;
  protected amount: bigint;
  protected feeAddress: string;
  protected feeAmount: bigint;

  constructor(transfer: BitcoinTransferParams, config: TConfig) {
    super(transfer, config);
    this.destinationAddress = transfer.destinationAddress;
    this.amount = transfer.amount;
    this.publicKey = transfer.publicKey;
    this.typeOfAddress = transfer.typeOfAddress;
    this.network = transfer.network;
    this.changeAddress = transfer.changeAddress;
    this.feeRate = transfer.feeRate;
    this.utxoData = transfer.utxoData;
    this.size = transfer.size;

    this.feeAddress = this.sourceDomain.feeAddress as string;
    this.feeAmount = BigInt((this.resource as BitcoinResource).feeAmount!);
  }

  getTransferTransaction(): BitcoinTransaction {
    return getPsbt(
      {
        source: this.sourceDomain.caipId,
        destination: this.destinationDomain.id,
        destinationAddress: this.destinationAddress,
        amount: this.amount,
        resource: this.resource.resourceId,
        utxoData: this.utxoData,
        publicKey: this.publicKey,
        typeOfAddress: this.typeOfAddress,
        network: this.network,
        feeRate: this.feeRate,
        changeAddress: this.changeAddress,
        size: this.size,
      },
      this.feeAddress,
      (this.resource as BitcoinResource).address,
      this.feeAmount,
    );
  }
}
