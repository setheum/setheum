---
slug: /protocol/ecosystem/evm
id: protocol-evm
title: EVM Support
description: The following details EVM support on Sygma.
---

:::info
The following section details Sygma's compatibility with EVM chains via its ecosystem of smart contracts. 
:::

# Introduction To EVM Support

Sygma uses Solidity smart contracts to enable transfers to and from Ethereum Virtual machine (EVM) compatible chains. These contracts consist of a core bridge contract (`Bridge.sol`), and a set of handler contracts (e.g. `ERC20Handler.sol`, `ERC721Handler.sol`, `PermissionlessGenericHandler.sol`, `FeeHandlerRouter.sol`, `BasicFeeHandler.sol`). [Different verification systems](../02-Tailored-Security/01-index.md) also require separate sets of smart contracts. These onchain components establish a relationship with the offchain actors via generation and detection of onchain contract events. 

## Sygma's Smart Contracts

### Bridge 

The bridge contract is responsible for initiating and executing proposed transfers. It facilitates and manages cross-chain transfer of assets and messages by recording and verifying `deposit`, `executeProposals`, and other cross-chain events. The actual handling of the deposits/withdrawals is handled by the appropriate handler contracts. The `Bridge.sol` contract serves as the primary gateway for a cross-chain interaction. 

### Handlers

The handlers are used by the bridge contract to interact with other existing contracts. For example, the `ERC20Handler` contract handles fungible ERC-20 tokens, enabling their deposit, withdrawal, and management within the protocol. On the other hand, the `PermissionlessGenericHandler` contract processes generic message deposits and their execution. It is designed to handle complex data encoding for executing transactions across chains.

Beyond contracts handling asset/message types, there are handler contracts that route fees (`FeeHandlerRouter`) and collect/manage fees (`BasicFeeHandler`, `PercentageFeeHandler`) based on the destination domain `sygmaID` and `sygmaResourceID` (see [Configuring Sygma](../01-index.md#configuring-sygma)).

For more information on both the Bridge and Handler contracts, please visit the [sygma-solidity](https://github.com/sygmaprotocol/sygma-solidity) repo.

### Spectre

Sygma's zk verification system, [Spectre](../02-Tailored-Security/03-Spectre/01-spectre-intro.md), implements a separate set of smart contracts, including the Bridge and Handler contracts discussed previously. Additionally, Spectre implements Spectre-specific smart contracts such as `Spectre`, the `SpectreProxy`, `AccessControlSegregator`, `Router`, and `Executor`.  Crucially, the `Spectre` and `SpectreProxy` contracts follow Ethereum's sync committee rotations, allowing lightweight verification of source computation. On the other hand, the `Executor`contract is responsible for receiving 1) block header zk-SNARK proofs from the offchain Spectre relayer node and 2) execution submissions and inclusion proofs from the offchain inclusion prover, and then finally 3) "executing" on a cross-chain interaction. 

For more information on these contracts, please visit the [sygma-x-solidity](https://github.com/sygmaprotocol/sygma-x-solidity) repo.