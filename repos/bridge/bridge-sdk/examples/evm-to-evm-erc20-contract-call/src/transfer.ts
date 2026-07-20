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
  Config,
  EvmResource,
  getSygmaScanLink,
  type Eip1193Provider,
} from "@buildwithsygma/core";
import {
  createFungibleAssetTransfer,
  FungibleTransferParams,
} from "@buildwithsygma/evm";
import dotenv from "dotenv";
import { Wallet, ethers, providers } from "ethers";
import Web3HttpProvider from "web3-providers-http";
import { getContractAddress, getContractInterface } from "./contracts";

dotenv.config();

const privateKey = process.env.PRIVATE_KEY;

if (!privateKey) {
  throw new Error("Missing environment variable: PRIVATE_KEY");
}

const SEPOLIA_CHAIN_ID = 11155111;
const BASE_SEPOLIA_CHAIN_ID = 84532;
const RESOURCE_ID =
  "0x0000000000000000000000000000000000000000000000000000000000001200";
const SEPOLIA_RPC_URL =
  process.env.SEPOLIA_RPC_URL || "https://ethereum-sepolia-rpc.publicnode.com";

const explorerUrls: Record<number, string> = {
  [SEPOLIA_CHAIN_ID]: "https://sepolia.etherscan.io",
};
const getTxExplorerUrl = (params: {
  txHash: string;
  chainId: number;
}): string => `${explorerUrls[params.chainId]}/tx/${params.txHash}`;

export async function erc20Transfer(): Promise<void> {
  const web3Provider = new Web3HttpProvider(SEPOLIA_RPC_URL);
  const ethersWeb3Provider = new providers.Web3Provider(web3Provider);
  const wallet = new Wallet(privateKey ?? "", ethersWeb3Provider);
  const sourceAddress = await wallet.getAddress();
  const destinationAddress = await wallet.getAddress();
  const contract = "sprinterNameService";

  const config = new Config();
  await config.init(process.env.SYGMA_ENV);
  const resource = config
    .getResources(BASE_SEPOLIA_CHAIN_ID)
    .find((resource) => resource.resourceId === RESOURCE_ID);

  if (!resource) return;

  const targetContractAddress = getContractAddress(
    BASE_SEPOLIA_CHAIN_ID,
    contract
  );

  const contractInterface = getContractInterface(contract);

  const params: FungibleTransferParams = {
    source: SEPOLIA_CHAIN_ID,
    destination: BASE_SEPOLIA_CHAIN_ID,
    sourceNetworkProvider: web3Provider as unknown as Eip1193Provider,
    resource: RESOURCE_ID,
    amount: BigInt(1) * BigInt(1e6),
    recipientAddress: ethers.constants.AddressZero,
    sourceAddress: sourceAddress,
    optionalGas: BigInt(5_000_000),
    environment: process.env.SYGMA_ENV,
    optionalMessage: {
      receiver: destinationAddress,
      transactionId: ethers.utils.formatBytes32String("EVM-ERC20+GENERIC"),
      actions: [
        {
          approveTo: targetContractAddress,
          tokenSend: (resource as EvmResource).address,
          tokenReceive: ethers.constants.AddressZero,
          nativeValue: BigInt(0),
          callTo: targetContractAddress,
          data: contractInterface.encodeFunctionData("claimName", [
            "EVM-ERC20+GENERIC",
            destinationAddress,
            BigInt(5) * BigInt(1e5),
          ]),
        },
      ],
    },
  };

  const transfer = await createFungibleAssetTransfer(params);

  const approvals = await transfer.getApprovalTransactions();
  console.log(`Approving Tokens (${approvals.length})...`);
  for (const approval of approvals) {
    const response = await wallet.sendTransaction(approval);
    await response.wait();
    console.log(
      `Approved, transaction: ${getTxExplorerUrl({ txHash: response.hash, chainId: SEPOLIA_CHAIN_ID })}`
    );
  }

  const transferTx = await transfer.getTransferTransaction();
  const response = await wallet.sendTransaction(transferTx);
  await response.wait();
  console.log(
    `Depositted, transaction:  ${getSygmaScanLink(response.hash, process.env.SYGMA_ENV)}`
  );
}

erc20Transfer().finally(() => {});
