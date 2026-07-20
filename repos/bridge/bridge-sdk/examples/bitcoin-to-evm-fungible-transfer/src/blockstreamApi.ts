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

import { TypeOfAddress } from "@buildwithsygma/bitcoin";
import type { BitcoinTransferParams } from "@buildwithsygma/bitcoin";
import type { Network, Signer } from "bitcoinjs-lib";
import { payments, Psbt } from "bitcoinjs-lib";

type SizeCalculationParams = {
  utxoData: BitcoinTransferParams["utxoData"];
  network: Network;
  publicKey: Buffer;
  depositAddress: string;
  domainId: number;
  amount: bigint;
  feeValue: bigint;
  changeAddress: string;
  signer: Signer;
  typeOfAddress: TypeOfAddress;
};

/**
 * Ee calculate the size of the transaction by using a tx with zero fee => input amount == output amount
 * Correctnes of the data is not relevant here, we need to know what's the size is going to be for the amount of inputs passed and the 4 outputs (deposit, change, fee, encoded data) we use to relay the funds
 */
export const calculateSize = ({
  utxoData,
  network,
  publicKey,
  depositAddress,
  domainId,
  amount,
  feeValue,
  changeAddress,
  signer,
  typeOfAddress,
}: SizeCalculationParams): number => {
  const pstb = new Psbt({ network: network });

  const scriptPubKey: Buffer = (typeOfAddress !== TypeOfAddress.P2TR)
    ? payments.p2wpkh({ pubkey: publicKey, network: network }).output as Buffer
    : payments.p2tr({ internalPubkey: publicKey, network: network }).output as Buffer;

  utxoData.forEach((utxo) => {
    const input = {
      hash: utxo.utxoTxId,
      index: utxo.utxoOutputIndex,
      witnessUtxo: {
        value: Number(utxo.utxoAmount),
        script: scriptPubKey,
      },
    };

    if (typeOfAddress === TypeOfAddress.P2TR) {
      (input as any).tapInternalKey = publicKey;
    }

    pstb.addInput(input);
  });


  const outputs = [
    {
      script: payments.embed({
        data: [Buffer.from(`${depositAddress}_${domainId}`)],
      }).output as Buffer,
      value: 0,
    },
    {
      address: changeAddress,
      value: Number(amount),
    },
    {
      address: changeAddress,
      value: Number(feeValue),
    },
    {
      address: changeAddress,
      value: 0,
    }
  ];

  outputs.forEach(output => pstb.addOutput(output));

  pstb.signAllInputs(signer);
  pstb.finalizeAllInputs();
  return pstb.extractTransaction(true).virtualSize();
};
