---
slug: /sdk/quickstart/substrate-token-transfer
id: quickstart-substrate-token-transfer
title: Substrate Token Transfers
description: Section that describes how to transfer token from or to a Substrate network.
sidebar_position: 3
draft: false
---

### Transferring a fungible asset from Substrate to EVM

Transferring assets from Substrate-based chains to EVM-based chains can be achieved using the Sygma SDK.

To facilitate the transfer, the following steps are required:

1. Create an instance of the `SubstrateAssetTransfer` object and initialize it
2. Determine the fee for the transfer, using the SubstrateAssetTransfer `getFee()` method
3. Prepare, sign, and send the `Transfer` transaction to the Substrate node

#### 1. Initialize the SubstrateAssetTransfer object

To initialize the asset transfer object, the following parameters need to be supplied:

- An instance of the PolkadotJS ApiPromise object
- The environment in which the bridge should function

```ts
const assetTransfer = new SubstrateAssetTransfer();

const wsProvider = new WsProvider("wss://URL-TO-YOUR-SUBSTRATE-INSTANCE");

// Create an instance of the PolkadotJS ApiPromise
const api = await ApiPromise.create({ provider: wsProvider });

await assetTransfer.init(
  api,
  Environment.TESTNET  // (i.e. DEVNET, TESTNET, MAINNET)
);
```

#### 2. Get fee

To facilitate the transfer of tokens, a fee must be attached. This fee can be determined by utilizing the asset transfer `GetFee(transfer)` method. You will need to know the destination ChainID as well as the ResourceID that has been configured on the bridge. These details can be determined by inspecting the configurations of the bridge (see [here](https://docs.buildwithsygma.com/environments))


```ts

const keyring = new Keyring({ type: "sr25519" });
await cryptoWaitReady();
const account = keyring.addFromUri(MNEMONIC); // use the dotenv module to pull in a mnemonic from a .env file
const transfer = await assetTransfer.createFungibleTransfer(
  account.address,
  DESTINATION_CHAINID,
  DESTINATION_ADDRESS,
  RESOURCE_ID,
  "AMOUNT" // use appropriate decimal places based on the token and/or ecosystem you are operating in
)

const fee = await assetTransfer.getFee(transfer);

```

#### Prepare the transfer transaction

Now that the fee has been determined, the transaction to deposit assets into the bridge should be generated, signed, and broadcast to the network.

```ts
// Build the transfer transaction
 const transferTx = assetTransfer.buildTransferTransaction(
  transfer, 
  fee
  );

// Sign and broadcast the transfer transaction
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
});
```

A full example of the above can be found [here](https://github.com/sygmaprotocol/sygma-sdk/blob/main/examples/substrate-to-evm-fungible-transfer/src/transfer.ts)