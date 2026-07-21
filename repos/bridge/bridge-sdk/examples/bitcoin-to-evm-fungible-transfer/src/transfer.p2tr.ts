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

import {
  createBitcoinFungibleTransfer,
  TypeOfAddress,
  getPublicKey,
  broadcastTransaction,
  fetchUTXOS,
  getFeeEstimates,
  processUtxos,
} from "@buildwithsygma/bitcoin";
import type { BitcoinTransferParams, UTXOData } from "@buildwithsygma/bitcoin";
import { BIP32Factory } from "bip32";
import { initEccLib, networks, Signer } from "bitcoinjs-lib";
import dotenv from "dotenv";
import * as tinysecp from "tiny-secp256k1";

import { calculateSize } from "./blockstreamApi.js";
import { Environment } from "@buildwithsygma/core";

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

  const { tweakedSigner, publicKeyDropedDERHeader } = (await getPublicKey({
    bip32,
    mnemonic: MNEMONIC as string,
    derivationPath: DERIVATION_PATH as string,
    network: networks.testnet,
    typeOfAddress: TypeOfAddress.P2TR,
  })) as { publicKeyDropedDERHeader: Buffer; tweakedSigner: Signer };

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
    publicKey: publicKeyDropedDERHeader,
    depositAddress: ADDRESS as unknown as string,
    domainId: DESTINATION_CHAIN_ID,
    amount: BigInt(AMOUNT!),
    feeValue: BigInt(0),
    changeAddress: ADDRESS as unknown as string,
    signer: tweakedSigner,
    typeOfAddress: TypeOfAddress.P2TR,
  });

  const transferParams: BitcoinTransferParams = {
    source: SOURCE_CAIPID!,
    destination: DESTINATION_CHAIN_ID,
    destinationAddress: DESTINATION_ADDRESS!,
    amount: BigInt(AMOUNT!),
    resource: RESOURCE_ID!,
    utxoData: mapedUtxos,
    feeRate: BigInt(Math.ceil(feeRate)),
    publicKey: publicKeyDropedDERHeader,
    typeOfAddress: TypeOfAddress.P2TR,
    network: networks.testnet,
    changeAddress: ADDRESS,
    size: BigInt(size),
    environment: Environment.TESTNET,
    sourceAddress: "",
  };

  const transfer = await createBitcoinFungibleTransfer(transferParams);

  const psbt = transfer.getTransferTransaction();

  console.log("Signing the transaction");

  psbt.signAllInputs(tweakedSigner);
  psbt.finalizeAllInputs();

  console.log("Extracting the transaction");
  const tx = psbt.extractTransaction(true);
  console.log("Transaction hex", tx.toHex());

  const txId = await broadcastTransaction(process.env.SYGMA_ENV, tx.toHex());
  console.log("Transaction broadcasted", `${EXPLORER_URL}/tx/${txId}`);
}

btcToEvmTransfer().finally(() => {});
