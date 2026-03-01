# Sygma Bridge Integration Guide: Testing & Deployment on Setheum

This guide provides instructions on how to set up, run, and test the Sygma bridge integration on the Setheum blockchain. It covers running the bridge locally alongside Ethereum testnets (e.g., Ganache, Hardhat, Truffle) and executing end-to-end token transfers using the Sygma SDK.

## Table of Contents
1. [Prerequisites](#prerequisites)
2. [Local Bridge Environment Setup](#local-bridge-environment-setup)
3. [Testing with Local Ethereum Chains (Ganache/Truffle)](#testing-with-local-ethereum-chains-ganache-truffle)
4. [Setting Up the Sygma SDK](#setting-up-the-sygma-sdk)
5. [Executing Cross-Chain Transfers](#executing-cross-chain-transfers)
6. [Interacting with the Chains Directly](#interacting-with-the-chains-directly)

---

## 1. Prerequisites

Before starting the local deployment, ensure you have the following installed on your machine:
- **Docker** and **Docker Compose**
- **Node.js** (v16+) and **Yarn**
- **Git**
- **Make**

---

## 2. Local Bridge Environment Setup

The local setup uses Docker to spin up a preconfigured environment containing:
- Two local EVM networks (simulating Ethereum/EVM chains, named `EVM1` and `EVM2`).
- One local Substrate network (simulating Setheum).
- Three relayer instances listening for, voting on, and executing bridging events.
- A fee oracle instance.

### Step 1: Clone the Sygma Relayer
First, clone the `bridge-relayer` repository (or the upstream Sygma relayer) into your workspace:

```bash
# If using the integrated Setheum version:
cd repos/bridge/bridge-relayer

# Or if using upstream directly:
# git clone https://github.com/sygmaprotocol/sygma-relayer.git
# cd sygma-relayer
```

### Step 2: Start the Local Setup
Start the containerized local environment using `make`:

```bash
make example
```

> **Note**: This command requires several gigabytes of disk space as it pulls multiple Docker images and spins up blockchain nodes. Ensure Docker is running before executing.

---

## 3. Testing with Local Ethereum Chains (Ganache / Truffle)

The local setup (`make example`) automatically spins up two local EVM networks running on Ganache containing pre-deployed Sygma bridge contracts.

However, if you wish to deploy the bridge contracts to your own local Truffle, Ganache, or Hardhat instance, you can use the `bridge-x-solidity` repository.

### Manual EVM Contract Deployment

1. **Navigate to the Solidity Repository**:
   ```bash
   cd repos/bridge/bridge-x-solidity
   ```

2. **Install Dependencies**:
   ```bash
   yarn install
   ```

3. **Configure your Local Network**:
   Update `truffle-config.js` or `hardhat.config.ts` to point to your running Ganache/Truffle instance (typically `http://127.0.0.1:8545`).

4. **Deploy the Contracts**:
   Deploy the core bridge contracts, handlers (e.g., ERC20, ERC721), and fee handlers to your local chain.
   ```bash
   yarn truffle migrate --network development
   # or
   yarn hardhat run scripts/deploy.ts --network localhost
   ```

---

## 4. Setting Up the Sygma SDK

To programmatically interact with the bridge and initiate transfers, you will use the Sygma SDK. 

### Step 1: Clone and Build the SDK

Navigate to the SDK directory in the Setheum bridge folder:

```bash
cd repos/bridge/bridge-sdk
yarn install
yarn sdk:build
```

### Step 2: Initialize the SDK for Local Environment

When writing test scripts to interact with the local setup, be sure to initialize the SDK's `assetTransfer` object with the `LOCAL` environment configuration:

```typescript
import { EVMAssetTransfer, Environment } from "@buildwithsygma/sygma-sdk-core";

const assetTransfer = new EVMAssetTransfer();
await assetTransfer.init(provider, Environment.LOCAL);
```

---

## 5. Executing Cross-Chain Transfers

The SDK repository includes pre-configured examples for running end-to-end transfers between EVM and Substrate (Setheum).

Navigate to the local transfer example folder:
```bash
cd repos/bridge/bridge-sdk/examples/local-fungible-transfer
```

### EVM to Substrate (Setheum) Transfer
To initiate an ERC-20 token transfer from the local EVM network (`EVM1`) to the local Substrate node (`Setheum`):

```bash
yarn run transfer:evm-substrate
```

### Substrate (Setheum) to EVM Transfer
To send the transferred tokens back from the Substrate node to the EVM network:

```bash
yarn run transfer:substrate-evm
```

These scripts utilize `ethers.js` and `@polkadot/api` combined with the Sygma SDK to construct the transfer transactions, wait for relayer confirmations, and verify the balances on the destination chains.

---

## 6. Interacting with the Chains Directly

Once the local setup (`make example`) is running, all node RPC endpoints are exposed locally. You can connect your wallet (e.g., MetaMask), development environments (Truffle/Hardhat), or blockchain explorers to these nodes.

| Network             | RPC Endpoint               | Notes |
| ------------------- | -------------------------- | ----- |
| **Local EVM 1**     | `http://127.0.0.1:8545`    | Preconfigured with Sygma EVM contracts |
| **Local EVM 2**     | `http://127.0.0.1:8547`    | Preconfigured with Sygma EVM contracts |
| **Local Substrate** | `ws://127.0.0.1:9944`      | Simulates Setheum with Sygma bridge pallets |

### Verifying Substrate Transactions
To verify the state of the Substrate network (Setheum) and check extrinsics manually:
1. Open the [Polkadot.js Apps Explorer](https://polkadot.js.org/apps/#/explorer).
2. Click the network selector in the top-left corner.
3. Scroll down to **Development** and select **Local Node (`ws://127.0.0.1:9944`)**.
4. Click **Switch**. You can now inspect recent blocks, extrinsics, and bridge events on the local Setheum chain.
