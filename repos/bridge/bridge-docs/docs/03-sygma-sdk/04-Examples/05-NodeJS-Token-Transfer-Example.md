---
slug: /sdk/quickstart/examples/node-js-example
id:  examples-nodejs-example
title: Node.js EVM To EVM Token Transfer Example
description: Section that describes how to transfer a token between EVM chains using nodeJS.
sidebar_position: 5
draft: true
---

:::info
In the following example, we will setup and demonstrate a cross-chain ERC-20 token transfer between the same address from Mumbai to Sepolia. This will be shown through the command line in a backend Node.js environment using the Sygma SDK.
:::

#### Prerequisites

- Install [Node.js](https://nodejs.org/en/about)(version 16 or above) for the backend JavaScript runtime environment.
- Obtain an API key for the Ethereum `Mumbai` testnet. An `API_KEY` will be required to initialize the `JsonRpcProvider` object with which blockchain interactions run through. You can sign up to obtain one for free with either [Infura](https://app.infura.io/) or [Alchemy](https://dashboard.alchemy.com/).
- [MetaMask](https://metamask.io/) should be installed, and an account **used only for development purposes** should be created. [Export the `private key` from MetaMask](https://support.metamask.io/hc/en-us/articles/360015289632-How-to-export-an-account-s-private-key).
- Obtain some `ERC20LRTest` tokens from the [Sygma faucet](https://faucet-ui-stage.buildwithsygma.com/). You will also need [MATIC](https://maticfaucet.com/) for gas.

:::info
The following example is written in `TypeScript`. You can use `CommonJS` with our SDK if you prefer to use standard `Node.js` for the script.
:::

#### Create folders for your project

Create a new root directory for your project, e.g. `/sygma_test_project`. Using a code editor, navigate to the newly created folder in your terminal `cd sygma_test_project`. Install the Sygma SDK by following the instructions on [Installing The SDK](https://docs.buildwithsygma.com/sdk/quickstart/install).

#### Initialize the project and install dependencies

Initialize the project by running the following commands:

```bash
yarn init -y
```

```bash
yarn add -D typescript dotenv ts-node
```

```bash
yarn add ethers@5.7.2
```

Create a `tsconfig.json` file in the root directory of your project and paste in the following code. This configuration file specifies the root files and the compiler options needed to compile the project. Specifically for this example, we will configure the target to ESNext, enable ES module loading, and set the output directory to dist.

```json
{
  "compilerOptions": {
    "composite": true,
    "target": "ESNext",
    "module": "CommonJS",
    "allowJs": true,
    "declaration": true,
    "sourceMap": true,
    "declarationMap": true,
    "resolveJsonModule": true,
    "skipLibCheck": true,
    "strict": true,
    "esModuleInterop": true,
    "downlevelIteration": true,
    "allowSyntheticDefaultImports": true,
    "forceConsistentCasingInFileNames": true,
    "moduleResolution": "node",
    "outDir": "dist",
    "baseUrl": "src"
  },
  "ts-node": {
    "esm": true,
    "experimentalSpecifierResolution": "node"
  },
  "include": ["src"]
}
```

Next, create a `src` folder in the root directory of your project.
Inside the `src` folder, create a `transfer.ts` file and an `index.ts` file.

Finally, because we are dealing with sensitive data such as a private key and an API key, we use environment variables to manage this data. Create a `.env` file in the root directory of your project. Paste in the following:

```
PROVIDER_API_KEY=your_provider_api_key_here
PRIVATE_KEY=your_private_key_here
```

Copy and paste (in quotations) the `Matic` API key obtained from Infura or Alchemy, as well as the exported private key from MetaMask.

:::danger
Remember that your private key should **never** be exposed. Hence, **DO NOT** commit the `.env` file to any repository and add it to your `.gitignore` file. Exposing the private key could result in complete loss of your funds. **_For this example, please only paste in the exported private keys of a wallet that is designated for testing/development purposes_**.
:::

### Usage

#### Imports and environment variable checks

We will begin by importing the necessary classes and functions from the `@buildwithsygma/sygma-sdk-core` and `ethers` packages. We also import the `dotenv` module, which loads the environment variables from the `.env` file created earlier. This snippet then checks whether the environment variables are properly configured. Paste in the following snippet into the `transfer.ts` file:

```ts
// transfer.ts
import { EVMAssetTransfer, Environment } from "@buildwithsygma/sygma-sdk-core";
import { Wallet, providers, ethers } from "ethers";
import dotenv from "dotenv";

dotenv.config();

const providerApiKey = process.env.PROVIDER_API_KEY;
const privateKey = process.env.PRIVATE_KEY;

if (!providerApiKey || !privateKey) {
  throw new Error("Missing environment variables");
}
```

#### Set up testnets and token symbol variables

Next, we setup some variables used for the rest of the script, including the two Ethereum testnets this example will conduct the token transfer on (`Matic` and `Sepolia`) and the symbol of the test token (`ERC20LRTest`) we will be using. Paste in the following snippet into the same `transfer.ts` file:

```ts
// transfer.ts
const SEPOLIA_CHAIN_ID = 11155111;
const RESOURCE_ID =
  "0x0000000000000000000000000000000000000000000000000000000000000300";
```

#### Create `assetTransfer` object

Now, we create a function that will return an `assetTransfer` object. This is the object that we are going to use to make the transfer between `Matic` and `Sepolia`. Paste in the following snippet into the same `transfer.ts` file:

```ts
// transfer.ts
/**
 * Initializes the Sygma SDK asset transfer module.
 * @param provider The Ethereum provider to use.
 * @returns A Promise that resolves when the module is initialized.
 */
async function initAssetTransfer(
  provider: providers.JsonRpcProvider,
): Promise<EVMAssetTransfer> {
  const assetTransfer = new EVMAssetTransfer();
  await assetTransfer.init(provider, Environment.TESTNET);
  return assetTransfer;
}
```

#### Create the main `erc20Transfer` function

Now, we create the main `erc20Transfer` function. This function uses all of the functions defined above, along with the Sygma SDK to perform the cross-chain token transfer.

You should have already obtained `ERC20LRTest` tokens from the faucet for this example. The line `ethers.utils.parseEther("50").toString()` is hardcoded to send 50 `ERC20LRTest` tokens.

Note that to send funds from network to another, we use this utility method called `createFungibleTransfer` to create the `transfer` object that we are going to use to bridge our tokens.

Paste in the following snippet into the same `transfer.ts` file:

```ts
// transfer.ts
/**
 * Transfers ERC20 tokens between two Ethereum networks using the Sygma SDK.
 * @returns A Promise that resolves when the transfer is complete.
 */
export async function erc20Transfer(): Promise<void> {
  const provider = new providers.JsonRpcProvider(providerApiKey);
  const wallet = new Wallet(privateKey as string, provider);

  const assetTransfer = await initAssetTransfer(provider);

  const transfer = assetTransfer.createFungibleTransfer(
    await wallet.getAddress(),
    SEPOLIA_CHAIN_ID,
    await wallet.getAddress(),
    RESOURCE_ID,
    50, // instructions to send 50 tokens
  );

  const fee = await assetTransfer.getFee(transfer);
  const approvals = await assetTransfer.buildApprovals(transfer, fee);
  for (const approval of approvals) {
    await wallet.sendTransaction(approval as providers.TransactionRequest);
  }

  const transferTx = await assetTransfer.buildTransferTransaction(
    transfer,
    fee,
  );

  const response = await wallet.sendTransaction(
    transferTx as providers.TransactionRequest,
  );
  console.log("Sent transfer with hash: " + response.hash);
}
```

#### Setup `index.ts`

Next, we setup the main entry point of the example. We import the `erc20Transfer` function from above and call it inside a try/catch block. Paste in the following snippet into the `index.ts` file:

```ts
// index.ts
import { erc20Transfer } from "./transfer";

async function main(): Promise<void> {
  try {
    await erc20Transfer();
    console.log("Transfer complete");
  } catch (error) {
    console.error(error);
  }
}

main();
```

#### Run the function

Run this example in the command line with:

```bash
npx ts-node ./src/index.ts
```

If done correctly, the console log should return a `Transfer complete` message along with a transaction hash. The `Deposit` event for the `ERC20LRTest` token should appear shortly on `Matic`, and the transferred tokens should appear shortly in the same address on `Sepolia`!

Congratulations on successfully performing a cross-chain token transfer using the Sygma SDK!

#### Another way to run the function

Alternatively, you can transpile the above by adding a build script into the `package.json` file:

```json
"scripts": {
    "build": "tsc"
  },
```

Then you can run it from the terminal by first running `yarn build`:

```bash
yarn build
```

Then, enter the following into the command line:

```bash
node ./dist/index.js
```
