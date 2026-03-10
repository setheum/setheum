---
slug: /sdk/advanced/localsetup
id: advanced-localsetup
title: Setting Up Locally
description: The following section details how to setup Sygma locally.
sidebar_position: 1
draft: false
---

:::info
You will need to have Docker and Docker Compose installed and running on your machine for the following example, which preconfigures a number of blockchain networks and the relayer infrastructure locally to demonstrate how each entity might interact in testing and production environments. Feel free to clone this repo to tinker with the setup. The full example can be found [here](https://github.com/sygmaprotocol/sygma-sdk/tree/main/examples/local-fungible-transfer).
:::

### Local Sygma Setup

The local setup contains preconfigured resources including:
- two EVM networks running on Ganache with Sygma contracts already deployed
  - The networks are named `EVM1` and `EVM2` respectively
- one Substrate network with Sygma pallets already implemented
  - The network is named `Substrate`
- three relayer instances listening for, voting, and executing on events 
- a fee oracle 

More details can be found in the local configuration. 

To interact with the local setup using the Sygma SDK, select `LOCAL` environment on initialization of the `assetTransfer` object.

```ts
const assetTransfer = new EVMAssetTransfer();
await assetTransfer.init(provider, Environment.LOCAL);
```

### Running the local setup example

We will be running the local setup example provided inside the [Sygma SDK repository](https://github.com/sygmaprotocol/sygma-sdk/tree/main/examples/local-fungible-transfer).

#### 1) Clone the Sygma Relayer repository

First, clone the Sygma relayer repository into a directory of your choice:

```
git clone https://github.com/sygmaprotocol/sygma-relayer.git
```

#### 2) Start local setup

`cd` into the cloned sygma-relayer folder, and then run the following command, which will start the dockerized setup: 

```
make example
```

:::note Be aware
The `make example` will require several GB's of space on your machine while it is running. You can follow this [documentation](https://docs.docker.com/engine/reference/commandline/system_prune/) to prune unused containers, networks, images, and volumes after you are done with the example in full.
:::

#### 3) Clone the Sygma SDK repository

Next, clone the Sygma SDK repository into a directory of your choice, and then `cd` into that folder:

```bash
git clone git@github.com:sygmaprotocol/sygma-sdk.git
cd sygma-sdk/
```

#### 4) Build the SDK

If you haven't already done so, install dependencies with:

```
yarn install
```

And build the SDK by running:

```
yarn sdk:build
```

#### 4) Run the EVM-to-Substrate token transfer example

`cd` into the `local-fungible-transfer` example inside the `/sygma-sdk/examples` folder, and run:

```bash
yarn run transfer:evm-substrate
```

This will start a local ERC-20 transfer of the `ERC20LRTest` token using the local Sygma setup. It will use `ethers` together with the sygma-sdk to create a transfer from the `EVM1` network to the `Substrate` network.

#### 5) Run the Substrate-to-EVM token transfer example

To demonstrate the exact same transfer in the other direction, you can also run:

```bash
yarn run transfer:substrate-evm
```

Similarly, this will use `@polkadot/api` together with the sygma-sdk to create an `ERC20LRTest` transfer from the `Substrate` network to the `EVM1` network.

:::note
To easily verify the bridging transactions, we have added console logs that will print out the transaction hashes and token balances. Additionally, you can verify the Substrate transactions (called `extrinsics` in the Dotsama ecosystem) by connecting to the locally hosted node using [Polkadot.js](https://polkadot.js.org/apps/#/explorer). This will give you access to the Polkadot.js block explorer. From here, you can paste in the Substrate transaction hashes to verify the chain state.
:::

<!-- #### Local configuration
TODO - add information on all registered resources on local setup -->

### Make changes to onchain Sygma setup
Once you start the local setup, it is possible to configure the Sygma EVM contracts or Sygma Substrate pallets, since all node RPC endpoints are exposed and you can interact with them as with any regular node.

| Network   | RPC endpoint               |
| --------- | ---------------------------|
| EVM1      | http://127.0.0.1:8545      |
| EVM2      | http://127.0.0.1:8547      |
| Substrate | ws://127.0.0.1:9944        |

