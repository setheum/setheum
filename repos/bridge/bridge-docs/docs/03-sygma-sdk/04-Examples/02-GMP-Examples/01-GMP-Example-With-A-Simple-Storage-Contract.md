---
slug: /sdk/examples/gmp/evm-example-gmp
id:  examples-gmp-evm-example-gmp
title: GMP Example With A Simple Storage Contract
description: An EVM to EVM example of GMP
sidebar_position: 1
draft: false
---

### GMP Example With A Simple Storage Contract

In the following example, we will use the `TESTNET` environment to pass a generic message from Ethereum Sepolia to Base-Sepolia Testnet using a simple storage contract. Specifically, the `deposit` method will be called on Sepolia, passing the details of the function to be called (the `store` function, or function signature `0xa271ced2`) on a smart contract deployed on Base-Sepolia ([0x669f52487ffa6f9abf722082f735537a98ec0e4b](https://sepolia.basescan.org/address/0x669f52487ffa6f9abf722082f735537a98ec0e4b)). The method will encode the current UNIX timestamp as the payload to be passed and stored in the destination chain contract. The data can be read by calling the `retrieve` function on the destination chain contract by querying the depositor address derived from the private key.

This is an example script that demonstrates the functionality of the Sygma SDK and the wider Sygma ecosystem of relayers and bridge and handler contracts. The complete example can be found in this [repo](
https://github.com/sygmaprotocol/sygma-sdk/tree/main/examples/evm-to-evm-generic-mesage-passing).

### Prerequisites

Before running the script, ensure that you have the following:

- Node.js v18
- Yarn (version 3.4.1 or higher)
- A development wallet funded with [Sepolia ETH](https://sepolia-faucet.pk910.de/) for gas 
- The [exported private key](https://support.metamask.io/hc/en-us/articles/360015289632-How-to-export-an-account-s-private-key) of your development wallet
- An Ethereum [provider](https://www.infura.io/) (in case the hardcoded RPC within the script does not work)

:::danger
We make use of the dotenv module to manage exported private keys with environment variables. Please note that accidentally committing a .env file containing private keys to a wallet with real funds, onto GitHub, could result in the complete loss of your funds. **Never expose your private keys.**
:::

### Getting Started

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

This example uses the `dotenv` module to manage private keys. To run the example, you will need to configure your environment variable to include your test development account's [exported private key](https://support.metamask.io/hc/en-us/articles/360015289632-How-to-export-an-account-s-private-key). A `.env.sample` is provided as a template.

**DO NOT COMMIT PRIVATE KEYS WITH REAL FUNDS TO GITHUB. DOING SO COULD RESULT IN COMPLETE LOSS OF YOUR FUNDS.**

Create a `.env` file in the evm-to-evm GMP example folder:

```bash
cd examples/evm-to-evm-generic-mesage-passing
touch .env
```

Replace between the quotation marks your exported private key:

`PRIVATE_KEY="YOUR_PRIVATE_KEY_HERE"`

To call the function on the destination chain contract, `cd` into the example folder `examples/evm-to-evm-generic-mesage-passing` and run:

```bash
cd examples/evm-to-evm-generic-mesage-passing
yarn run transfer
```

The example will use `ethers` in conjunction with the sygma-sdk to call a function on a smart contract on Base-Sepolia by calling the `Deposit` method on Sepolia and passing the details of the function to be called.

Replace the placeholder values in the `.env` file with your own Ethereum wallet private key and provider URL.

