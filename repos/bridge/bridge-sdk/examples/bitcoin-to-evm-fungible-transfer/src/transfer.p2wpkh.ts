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

import type { BitcoinTransferParams, UTXOData } from "@buildwithsygma/bitcoin";
import {
  createBitcoinFungibleTransfer,
  TypeOfAddress,
  getPublicKey,
  getFeeEstimates,
  fetchUTXOS,
  processUtxos,
  broadcastTransaction,
} from "@buildwithsygma/bitcoin";
import { BIP32Factory, BIP32Interface } from "bip32";
import { initEccLib, networks } from "bitcoinjs-lib";
import dotenv from "dotenv";
import * as tinysecp from "tiny-secp256k1";

import { calculateSize } from "./blockstreamApi.js";

dotenv.config();

const DESTINATION_CHAIN_ID = 11155111;
const {
  SOURCE_CAIPID,
  DESTINATION_ADDRESS,
  RESOURCE_ID,
  BLOCKSTREAM_URL,
  EXPLORER_URL,
  MNEMONIC,
  DERIVATION_PATH,
  ADDRESS,
  AMOUNT,
} = process.env;

if (
  !SOURCE_CAIPID ||
  !DESTINATION_ADDRESS ||
  !RESOURCE_ID ||
  !MNEMONIC ||
  !DERIVATION_PATH ||
  !ADDRESS ||
  !BLOCKSTREAM_URL ||
  !AMOUNT
) {
  throw new Error(
    "Missing required environment variables, please make sure .env file exists."
  );
}

async function btcToEvmTransfer(): Promise<void> {
  // pre setup
  initEccLib(tinysecp);
  const bip32 = BIP32Factory(tinysecp);
  console.log("Transfer BTC to EVM");

  const { derivedNode } = (await getPublicKey({
    bip32,
    mnemonic: MNEMONIC as string,
    derivationPath: DERIVATION_PATH as string,
    network: networks.testnet,
    typeOfAddress: TypeOfAddress.P2WPKH,
  })) as { derivedNode: BIP32Interface };

  const feeRate = await getFeeEstimates(process.env.SYGMA_ENV, "5");
  const utxos = await fetchUTXOS(
    process.env.SYGMA_ENV,
    ADDRESS as unknown as string
  );

  const processedUtxos = processUtxos(utxos, Number(BigInt(AMOUNT!)));

  const mapedUtxos = processedUtxos.map((utxo) => ({
    utxoTxId: utxo.txid,
    utxoOutputIndex: utxo.vout,
    utxoAmount: BigInt(utxo.value),
  })) as unknown as UTXOData[];

  const size = calculateSize({
    utxoData: mapedUtxos,
    network: networks.testnet,
    publicKey: derivedNode.publicKey,
    depositAddress: ADDRESS as unknown as string,
    domainId: DESTINATION_CHAIN_ID,
    amount: BigInt(AMOUNT!),
    feeValue: BigInt(0),
    changeAddress: ADDRESS as unknown as string,
    signer: derivedNode,
    typeOfAddress: TypeOfAddress.P2WPKH,
  }); // aprox estimation of the size of the tx

  const transferParams: BitcoinTransferParams = {
    source: SOURCE_CAIPID!,
    destination: DESTINATION_CHAIN_ID!,
    destinationAddress: DESTINATION_ADDRESS!,
    amount: BigInt(AMOUNT!),
    resource: RESOURCE_ID!,
    utxoData: mapedUtxos,
    publicKey: derivedNode.publicKey,
    typeOfAddress: TypeOfAddress.P2WPKH,
    network: networks.testnet,
    changeAddress: ADDRESS,
    feeRate: BigInt(Math.ceil(feeRate)),
    size: BigInt(size),
    sourceAddress: "",
  };

  const transfer = await createBitcoinFungibleTransfer(transferParams);

  const psbt = transfer.getTransferTransaction();

  console.log("Signing the transaction");

  psbt.signAllInputs(derivedNode);
  psbt.finalizeAllInputs();

  console.log("Extracting the transaction");
  const tx = psbt.extractTransaction(true);
  console.log("Transaction hex", tx.toHex());

  const txId = await broadcastTransaction(process.env.SYGMA_ENV, tx.toHex());
  console.log("Transaction broadcasted", `${EXPLORER_URL}/tx/${txId}`);
}

btcToEvmTransfer().finally(() => {});
