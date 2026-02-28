---
slug: /sdk/quickstart/gmp
id: quickstart-gmp
title: Generic Message Passing (GMP)
description: The following section details how to perform GMP transfers.
sidebar_position: 4
draft: false
---

:::info
In the examples below `Ethers` v5 was used. If you were to use v6, keep in mind the differences between versions.
:::

### Transferring a message between EVM chains

Generic messages can be transferred between EVM chains using the Sygma SDK.

To facilitate the transfer, the following steps are required:

1. Create an instance of the `EVMGenericMessageTransfer` object and initialize it.
2. Determine the fee for the transfer, using the  `EVMGenericMessageTransfer.getFee()` method
3. Prepare, sign, and send the Transfer transaction to the Source network node

The `executeDeposit` function prepares and populates a deposit transaction. The key parameter is `depositData`, which is a string requiring a specific format. Refer to the [Generic Message Passing](../../02-sygma-protocol/06-generic.md) documentation for instructions on how to format the `depositData` string correctly.

There are a few requirements for the Destination chain contract function that gets called. Refer to the [Generic Message Passing](../../02-sygma-protocol/06-generic.md) documentation for details.

#### 1. Initialize the EvmAssetTransfer object

To initialize the generic message transfer object, the following parameters need to be supplied:

- An instance of Ethers provider
- The environment in which the bridge should function

```ts
const messageTransfer = new EVMGenericMessageTransfer();

const sourceProvider = new providers.JsonRpcProvider(
  "https://URL-TO-YOUR-RPC"
);
const destinationProvider = new providers.JsonRpcProvider(
  "https://URL-TO-YOUR-RPC"
);

 await messageTransfer.init(
  sourceProvider, 
  Environment.TESTNET  // (i.e. DEVNET, TESTNET, MAINNET)
  );
```

#### 2. Get fee

To facilitate the transfer of a generic message, a fee must be paid. This fee can be determined by utilizing the `messageTransfer.getFee(transfer)` method. You will need to know the destination ChainID as well as the ResourceID that has been configured on the bridge. These details can be determined by inspecting the configurations of the bridge (see [here](https://docs.buildwithsygma.com/environments))


```ts
const wallet = new Wallet(
  privateKey ?? "", // use the dotenv module to pull in a private key from a .env file
  sourceProvider
  );

const transfer = messageTransfer.createGenericMessageTransfer(
  await wallet.getAddress(),
  DESTINATION_CHAINID,
  RESOURCE_ID,
  DESTINATION_CONTRACT_ADDRESS, // contract address you are calling to
  DESTINATION_FUNCTION_SIGNATURE, // function signature you are invoking cross-chain
  EXECUTION_DATA, // the actual data payload that the smart contract function is expecting 
  MAX_FEE
)

const fee = await messageTransfer.getFee(transfer);
```
### 3. Prepare, sign, and send the Transfer transaction to the Source network node

```ts
const transferTx = await messageTransfer.buildTransferTransaction(
    transfer,
    fee
  );

// Send the transaction using the wallet
 const response = await wallet.sendTransaction(
    transferTx as providers.TransactionRequest
  );
  console.log("Sent transfer with hash: ", response.hash);
```

A full example of the above can be found [here](https://github.com/sygmaprotocol/sygma-sdk/blob/main/examples/evm-to-evm-generic-mesage-passing/src/transfer.ts)