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

import type { Eip1193Provider, Environment } from "@buildwithsygma/core";
import { getSygmaScanLink } from "@buildwithsygma/core";
import { createCrossChainContractCall } from "@buildwithsygma/evm";
import dotenv from "dotenv";
import type { BigNumber } from "ethers";
import { Wallet, ethers, providers } from "ethers";
import Web3HttpProvider from "web3-providers-http";

import { sepoliaBaseStorageContract } from "./contracts/index.js";

dotenv.config();

const sygmaEnv = process.env.SYGMA_ENV as Environment;
const privateKey = process.env.PRIVATE_KEY;
const sleep = (ms: number): Promise<unknown> =>
  new Promise((r) => setTimeout(r, ms));

if (!privateKey) {
  throw new Error("Missing environment variable: PRIVATE_KEY");
}

// Base Sepolia
const DESTINATION_CHAIN_ID = 84532;
// Sepolia
const SEPOLIA_CHAIN_ID = 11155111;
// Permissionless Generic Transfer
const RESOURCE_ID =
  "0x0000000000000000000000000000000000000000000000000000000000000600";
// Contract address on destination chain (Base Sepolia)
const EXECUTE_CONTRACT_ADDRESS = "0x4bE595ab5A070663B314970Fc10C049BBA0ad489";
// Maximum Fee to be paid for this transfer
const MAX_FEE = "3000000";
// Destination RPC Url
const BASE_SEPOLIA_RPC_URL =
  process.env.BASE_SEPOLIA_RPC_URL || "https://sepolia.base.org";
// Source RPC Url
const SEPOLIA_RPC_URL =
  process.env.SEPOLIA_RPC_URL || "https://gateway.tenderly.co/public/sepolia";
// In order to support EIP1193, Web3HttpProvider is used from web3-providers-http@1.10.4
const sourceProvider = new Web3HttpProvider(SEPOLIA_RPC_URL);
// ethers provider (wrapping)
const ethersProvider = new ethers.providers.Web3Provider(sourceProvider);
// ethers wallet object using private key from env
const wallet = new Wallet(privateKey, ethersProvider);
// destination ethers JsonRpcProvider
const destinationProvider = new providers.JsonRpcProvider(BASE_SEPOLIA_RPC_URL);
const paramAddress = "0x98729c03c4D5e820F5e8c45558ae07aE63F97461" as const;

export async function genericMessage(): Promise<void> {
  // Wallet address from the specified
  // private key
  const walletAddress = await wallet.getAddress();
  // ethers contract on destination chain
  const destinationStorageContract = new ethers.Contract(
    EXECUTE_CONTRACT_ADDRESS,
    sepoliaBaseStorageContract,
    destinationProvider
  );
  // value set inside the contract before
  // cross chain tranfer
  const valueBeforeBridging =
    (await destinationStorageContract.callStatic.retrieve(
      walletAddress
    )) as BigNumber;
  console.log(`Value before update: ${valueBeforeBridging.toString()}`);
  // Specification of parameters
  // to initiate a cross chain contract
  // call
  const transfer = await createCrossChainContractCall<
    typeof sepoliaBaseStorageContract,
    "store"
  >({
    gasLimit: BigInt(0),
    functionParameters: [paramAddress, paramAddress, BigInt(3052070251)],
    functionName: "store",
    destinationContractAbi: sepoliaBaseStorageContract,
    destinationContractAddress: EXECUTE_CONTRACT_ADDRESS,
    maxFee: BigInt(MAX_FEE),
    source: SEPOLIA_CHAIN_ID,
    destination: DESTINATION_CHAIN_ID,
    sourceNetworkProvider: sourceProvider as unknown as Eip1193Provider,
    sourceAddress: walletAddress,
    resource: RESOURCE_ID,
    environment: process.env.SYGMA_ENV,
  });
  // transaction that can be sent to the chain
  const transaction = await transfer.getTransferTransaction();
  // sending the transaction using ethers js
  const tx = await wallet.sendTransaction(transaction);
  console.log("Sent transaction: ", tx.hash);
  const scannerUrl = getSygmaScanLink(tx.hash, sygmaEnv);
  await tx.wait();
  console.log("waiting for transaction indexing", tx.hash);
  await sleep(5000);
  console.log("Transaction link: ", scannerUrl);
}

genericMessage()
  .then(() => {
    process.exit(0);
  })
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
