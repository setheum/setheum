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

import type { BaseTransferParams } from '@buildwithsygma/core';
import type { BIP32API, BIP32Interface } from 'bip32';
import type { Network, networks, Psbt, Signer } from 'bitcoinjs-lib';

export enum TypeOfAddress {
  P2WPKH = 'P2WPKH',
  P2TR = 'P2TR',
}

export type UTXOData = {
  utxoTxId: string;
  utxoAmount: bigint;
  utxoOutputIndex: number;
};

export type CreateInputData = {
  utxoData: UTXOData;
  publicKey: Buffer;
  network: networks.Network;
  typeOfAddress: TypeOfAddress;
};

export interface BitcoinTransferParams extends BaseTransferParams {
  destinationAddress: string;
  amount: bigint;
  utxoData: UTXOData[];
  publicKey: Buffer;
  typeOfAddress: TypeOfAddress;
  network: networks.Network;
  feeRate: bigint;
  changeAddress?: string;
  size: bigint;
}

export type CreatePsbtParams = Pick<
  BitcoinTransferParams,
  | 'source'
  | 'destination'
  | 'destinationAddress'
  | 'amount'
  | 'resource'
  | 'utxoData'
  | 'publicKey'
  | 'typeOfAddress'
  | 'network'
  | 'feeRate'
  | 'changeAddress'
  | 'size'
>;

export type BitcoinTransaction = Psbt;

export type PaymentReturnData = {
  output: Buffer;
  address?: string;
};

export type BitcoinTransferInputData = {
  hash: string | Buffer;
  index: number;
  witnessUtxo: { value: number; script: Buffer };
  tapInternalKey?: Buffer;
};

export type PublicKeyParams = {
  bip32: BIP32API;
  mnemonic: string;
  derivationPath: string;
  network: Network;
  typeOfAddress: TypeOfAddress;
};

export type GetPublicKeyResult =
  | { tweakedSigner: Signer; publicKeyDropedDERHeader: Buffer }
  | { derivedNode: BIP32Interface };
