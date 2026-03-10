---
slug: /sdk/examples/erc20/substrate-evm-example
id:  examples-erc20-substrate-evm-example
title: Substrate To EVM Token Transfer
description: Section that describes how to perform a Substrate to EVM token transfer.
sidebar_position: 3
draft: false
---

:::warning
Please be aware that the Rococo-Phala testnet is currently down due to ongoing maintenance by the Phala team as they migrate their testnet to Paseo. Stay tuned for further updates.
:::


### EVM-to-Substrate token transfer example

In the following example, we will use the `TESTNET` environment to perform a cross-chain ERC-20 transfer with 0.5 sygUSD `sygUSD` tokens. The transfer will be initiated on the Substrate-side via the Rococo-Phala testnet and received on the EVM-side via the Sepolia Ethereum testnet.

This is an example script that demonstrates the functionality of the Sygma SDK and the wider Sygma ecosystem of relayers and bridge and handler contracts/pallets. The complete example can be found in this [repo](https://github.com/sygmaprotocol/sygma-sdk/tree/main/examples/substrate-to-evm-fungible-transfer).

### Prerequisites

Before running the script, ensure that you have the following:

- Node.js v18
- Yarn (version 3.4.1 or higher)
- The 12-word mnemonic for your Substrate development wallet
- An Ethereum wallet to receive tokens into (the example presets an existing wallet address already)
- A Substrate provider (in case the hardcoded WSS within the script does not work)
- A Substrate development wallet funded with `sygUSD` tokens

import App from '../../../../src/Faucet/App'; 

<App />
<br/>

:::danger
We make use of the dotenv module to manage Substrate's private mnemonics with environment variables. Please note that accidentally committing a .env file containing private mnemonics to a wallet with real funds, onto GitHub, could result in the complete loss of your funds. **Never expose your private keys.**
:::

### Getting started

1. Clone the repository 

Clone the sygma-sdk repository into a directory of your choice, and then `cd` into the folder:

```bash
git clone git@github.com:sygmaprotocol/sygma-sdk.git
cd sygma-sdk/
```

2. Install dependencies
   
Install the project dependencies by running:

```bash
yarn install
```

3. Build the SDK

Build the SDK by running the following command:

```bash
yarn sdk:build
```

4. Usage

This example uses the `dotenv` module to import the Substrate 12-word private mnemonic. To run the example, you will need to configure your environment variable to include your Substrate test development account's [12-word seed](https://support.polkadot.network/support/solutions/articles/65000169731-polkadot-extension-how-can-i-view-my-mnemonic-phrase-). A `.env.sample` is provided as a template.

**DO NOT COMMIT YOUR MNEMONIC WITH REAL FUNDS TO GITHUB. DOING SO COULD RESULT IN COMPLETE LOSS OF YOUR FUNDS.**

Create a `.env` file in the substrate-to-evm example folder:

```bash
cd examples/substrate-to-evm-fungible-transfer
touch .env
```

Replace between the quotation marks your 12-word mnemonic:

`PRIVATE_MNEMONIC="YOUR TWELVE WORD MNEMONIC HERE WITH SPACES"`

Replace the placeholder value in the script for `recipient` with your preferred destination EVM address.
   
To send a Substrate token transfer from Substrate to EVM, run:

```bash
cd examples/substrate-to-evm-fungible-transfer
yarn run transfer
```

The example will use `@polkadot/keyring` in conjunction with the sygma-sdk to create a transfer from Rococo-Phala to Sepolia with the `sygUSD` token. It will be received on Sepolia as a `sygUSD` token.

### Script functionality

This example script performs the following steps:

- Initializes the SDK by importing the required packages and defining the constants for the script.

```ts
import { Keyring } from "@polkadot/keyring";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import dotenv from "dotenv";
import {
  Environment,
  Substrate,
  getTransferStatusData,
  TransferStatusResponse
} from "@buildwithsygma/sygma-sdk-core";

const { SubstrateAssetTransfer } = Substrate;

const SEPOLIA_CHAIN_ID = 11155111;
const RESOURCE_ID =
  "0x0000000000000000000000000000000000000000000000000000000000001100"; // this is the resourceID for sygUSD 
const MNEMONIC = process.env.PRIVATE_MNEMONIC;
const recipient = "0xD31E89feccCf6f2DE10EaC92ADffF48D802b695C"; // replace this value for your preferred EVM recipient address 
const RHALA_RPC_URL = process.env.RHALA_RPC_URL || "wss://rhala-node.phala.network/ws"
if (!MNEMONIC) {
  throw new Error("Missing environment variable: PRIVATE_MNEMONIC");
}
```

- Configures the dotenv module and sets the `MNEMONIC` as a value to be pulled from the `.env` file.

```ts
import dotenv from "dotenv";

dotenv.config();

const MNEMONIC = process.env.PRIVATE_MNEMONIC;

if (!MNEMONIC) {
  throw new Error("Missing environment variable: PRIVATE_MNEMONIC");
}
```

- Defines the main Substrate transfer function, including the connection to the blockchain using a WebSocket provider, initializing the asset transfer instance, and setting up the keyring and account from the mnemonic phrase.

```ts
const substrateTransfer = async (): Promise<void> => {
  const keyring = new Keyring({ type: "sr25519" });
  // Make sure to fund this account with native tokens
  // Account address: 5FNHV5TZAQ1AofSPbP7agn5UesXSYDX9JycUSCJpNuwgoYTS

  await cryptoWaitReady();

  const account = keyring.addFromUri(MNEMONIC);

  const wsProvider = new WsProvider(RHALA_RPC_URL);
  const api = await ApiPromise.create({ provider: wsProvider });

  const assetTransfer = new SubstrateAssetTransfer();

  await assetTransfer.init(api, Environment.TESTNET);
```

- Invokes the `getTransferStatusData` and `getStatus` functions by taking the transaction hash as an input to periodically check the status of the cross-chain transaction.

```ts
const id = setInterval(() => {
      getStatus(status.asInBlock.toString())
        .then((data) => {
          if (data[0]) {
            console.log("Status of the transfer", data[0].status);
            if(data[0].status == "executed") {
              clearInterval(id);
              process.exit(0);
            }
          } else {
            console.log("Waiting for the TX to be indexed");
          }
        })
        .catch((e) => {
          console.log("error:", e);
        });
    }, 5000);
  });
};
```

- Constructs a transfer object that calculates the fee, then builds, signs, and sends the transaction.

```ts
const transferTx = assetTransfer.buildTransferTransaction(transfer, fee);

  const unsub = await transferTx.signAndSend(account, ({ status }) => {
    console.log(`Current status is ${status.toString()}`);

    if (status.isInBlock) {
      console.log(
        `Transaction included at blockHash ${status.asInBlock.toString()}`
      );
    } else if (status.isFinalized) {
      console.log(
        `Transaction finalized at blockHash ${status.asFinalized.toString()}`
      );
      unsub();
    }
```

- Logs the current status of the transaction, and if it's included in a block or finalized, outputs the respective block hash.

```ts
const unsub = await transferTx.signAndSend(account, ({ status }) => {
  console.log(`Current status is ${status.toString()}`);
  if (status.isInBlock) {
    console.log(`Transaction included at blockHash ${status.asInBlock.toString()}`);
  } else if (status.isFinalized) {
    console.log(`Transaction finalized at blockHash ${status.asFinalized.toString()}`);
    unsub();
  }
});
```