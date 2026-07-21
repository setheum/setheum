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

import type { SubstrateAssetTransferRequest } from "@buildwithsygma/substrate";
import { createSubstrateFungibleAssetTransfer } from "@buildwithsygma/substrate";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import dotenv from "dotenv";

dotenv.config();

const MNEMONIC = process.env.PRIVATE_MNEMONIC;
if (!MNEMONIC) {
  throw new Error("Missing environment variable: PRIVATE_MNEMONIC");
}

const TANGLE_CHAIN_ID = 3799;
const SEPOLIA_CHAIN_ID = 11155111;

const RECIPIENT_ADDRESS =
  process.env.RECIPIENT_ADDRESS || "0xE39bb23F17a2cf7C9a8C4918376A32036A8867db";
const RESOURCE_ID =
  "0x0000000000000000000000000000000000000000000000000000000000002000";
const SYGMA_EXPLORER_URL = "https://scan.test.buildwithsygma.com";
const TANGLE_RPC_URL =
  process.env.SOURCE_SUBSTRATE_RPC_URL ?? "wss://rpc.tangle.tools";

const getSygmaExplorerTransferUrl = (params: {
  blockNumber: number;
  extrinsicIndex: number;
}): string =>
  `${SYGMA_EXPLORER_URL}/transfer/${params.blockNumber}-${params.extrinsicIndex}`;

const substrateTransfer = async (): Promise<void> => {
  // Make sure to account with native tokens
  const keyring = new Keyring({ type: "sr25519" });
  await cryptoWaitReady();
  const account = keyring.addFromUri(MNEMONIC);
  const wsProvider = new WsProvider(TANGLE_RPC_URL);
  const api = await ApiPromise.create({ provider: wsProvider });

  const transferParams: SubstrateAssetTransferRequest = {
    source: TANGLE_CHAIN_ID,
    destination: SEPOLIA_CHAIN_ID,
    sourceNetworkProvider: api,
    sourceAddress: account.address,
    resource: RESOURCE_ID,
    amount: BigInt(1) * BigInt(1e18),
    recipientAddress: RECIPIENT_ADDRESS,
    environment: process.env.SYGMA_ENV,
  };

  const transfer = await createSubstrateFungibleAssetTransfer(transferParams);
  const transferTx = await transfer.getTransferTransaction();

  const unsub = await transferTx.signAndSend(account, (results) => {
    const { status } = results;
    console.log(`Current status is ${status.toString()}`);

    if (status.isInBlock) {
      console.log(
        `Transaction included at blockHash ${status.asInBlock.toString()}`
      );
    } else if (status.isFinalized) {
      const blockNumber = results.blockNumber?.toNumber();
      const extrinsicIndex = results.txIndex;

      if (blockNumber && extrinsicIndex) {
        console.log(
          `Transaction finalized at blockHash ${status.asFinalized.toString()}`
        );
        console.log(
          `Explorer URL: ${getSygmaExplorerTransferUrl({ blockNumber, extrinsicIndex })}`
        );
      }
      unsub();
      process.exit(0);
    } else if (status.isDropped || status.isInvalid || status.isUsurped) {
      console.error("Transaction failed. Status:", status.toString());
    }
  });
};

substrateTransfer()
  .catch((e) => console.log(e))
  .finally(() => {});
