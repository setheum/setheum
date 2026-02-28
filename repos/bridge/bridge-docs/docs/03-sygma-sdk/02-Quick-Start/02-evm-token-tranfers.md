---
slug: /sdk/quickstart/transfertoken
id: quickstart-transfertoken
title: EVM Token Transfers
description: The following section details how to perform a cross-chain token transfer.
sidebar_position: 2
draft: false
---

:::info
In the examples below `Ethers` v5 was used. If you were to use v6, keep in mind the differences between versions.
:::

### Transferring a fungible asset between EVM chains

Transferring assets between EVM-based chains can be achieved using the Sygma SDK.

To facilitate the transfer, the following steps are required:

1. Create an instance of the `EvmAssetTransfer` object and initialize it.
2. Determine the fee for the transfer, using the EvmAssetTransfer `getFee()` method
3. Check for any approvals required, and if required sign and broadcast these transactions.
4. Prepare, sign, and send the `Transfer` transaction to the Source network node


#### 1. Initialize the EvmAssetTransfer object

To initialize the asset transfer object, the following parameters need to be supplied:

- An instance of Ethers provider
- The environment in which the bridge should function 

```ts
const assetTransfer = new EvmAssetTransfer();

const provider = new JsonRpcProvider("https://URL-TO-YOUR-RPC")

await assetTransfer.init(
  provider,
  Environment.TESTNET // (i.e. DEVNET, TESTNET, MAINNET)
);
```

#### 2. Get fee

To facilitate the transfer of tokens, a fee must be attached. This fee can be determined by utilizing the asset transfer `GetFee(transfer)` method. You will need to know the destination ChainID as well as the ResourceID that has been configured on the bridge. These details can be determined by inspecting the configurations of the bridge (see [here](https://docs.buildwithsygma.com/environments))

```ts
const wallet = new Wallet(
  privateKey as string, // use the dotenv module to pull in a private key from a .env file
  provider
);

const transfer = await assetTransfer.createFungibleTransfer(
  await wallet.getAddress(),
  DESTINATION_CHAINID, 
  DESTINATION_ADDRESS, 
  RESOURCE_ID, 
  "AMOUNT" // use appropriate decimal places based on the token and/or ecosystem you are operating in
)

const fee = await assetTransfer.getFee(transfer);
```

#### 3. Check Approvals 

You can check if approvals are required for your transfer. If there are approvals required for the transfer and the `fee` has been obtained, you can sign the transaction and send it.

```ts
// Build approvals given the `transfer` and `fee` parameters
const approvals = await assetTransfer.buildApprovals(transfer, fee);

// Check if approvals are needed
for (const approval of approvals) {
    const response = await wallet.sendTransaction(
      approval as providers.TransactionRequest
    );
    console.log("Sent approval with hash: ", response.hash);
  }

// Build the transfer transaction
const transferTx = await assetTransfer.buildTransferTransaction(
    transfer,
    fee
  );

// Send the transaction using the wallet
const response = await wallet.sendTransaction(
    transferTx as providers.TransactionRequest
  );
```

#### Check Transaction Hash

The `response` object returned by the `sendTransaction` method contains a `hash` property. This transaction hash is logged to the console, and it can be used to look up the transaction on a block explorer.

```ts
// Print the transaction hash
  console.log("Sent transfer with hash: ", response.hash);
```

A full example of the above can be found [here](https://github.com/sygmaprotocol/sygma-sdk/blob/main/examples/evm-to-evm-fungible-transfer/src/transfer.ts)