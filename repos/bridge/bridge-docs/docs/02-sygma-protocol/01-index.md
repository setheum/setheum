---
slug: /protocol
id: protocol-index
title: Sygma Protocol
description: The following details the architecture of Sygma.
---

:::info
The following sections detail the architectural components of the Sygma protocol.
:::

The Sygma interoperability protocol is able to transfer both tokens and arbitrary data. This allows developers and users to transfer ERC-20 tokens, ERC-721 tokens, ERC-1155 tokens, and **[Generic data](./06-generic.md).** Generic data can be used to bridge governance proposals or voting actions, for example, or any other contract call by transferring [calldata](https://ethereum.stackexchange.com/questions/52989/what-is-calldata).

The Sygma protocol stack is designed to leverage the combined strength of both trusted and trustless verification systems. This ultimately forms a hybrid, trust-minimized system that facilitates secure communication and data exchange between blockchains driven by a user’s context and determined by tradeoffs around trust minimization, speed, and cost to transfer. Its modular design also facilitates its operation across any blockchain ecosystem and consensus mechanism with ease of integration. You can read more about this feature under [Tailored Security](./02-Tailored-Security/01-index.md).

![](<../../static/assets/sygma_protocol_stack.png>)

Sygma uses an offchain [relayer](./03-relayers.md) network to verify events on one chain that are to be posted to a destination chain. These relayer nodes, or agents, use a variety of verification systems to determine canon across chains. 

Currently, the Sygma protocol is compatible with [EVM](../02-sygma-protocol/04-Supported-Ecosystems/01-evm.md) and [Substrate](../02-sygma-protocol/04-Supported-Ecosystems/02-substrate.md)-based networks (i.e. Polkadot and Kusama), but is proven to be easily extended to other networks such as Bitcoin and Tendermint-based chains (i.e. Cosmos).

The below diagram describes a typical transfer flow within the Sygma protocol using MPC verification between two EVM-based networks: 

![](<../../static/assets/Bridging Diagram.png>)

## Configuring Sygma 

Sygma utilizes a shared configuration file to enable cross-chain communications. It allows efficient management of network-specific parameters like `sygmaID`s, `chainID`s, and `sygmaResourceID`s. The shared config is primarily used by various components within the Sygma protocol, such as relayers, widgets, and SDK examples.

- **`sygmaID`**: Formerly domainID, it refers to Sygma-specific identifiers ascribed to an L1 or L2 blockchain network. Most configurations in Sygma are domain-specific.

- **`caipId`**: Representing [caip2](https://chainagnostic.org/CAIPs/caip-2) compatible domain identifier

- **`ChainID`**: Represents the more conventionally understood identifiers denoting a blockchain network. It distinguishes between different networks within the same domain or across domains. It is primarily used during interactions with RPC endpoints. 

- **`sygmaResourceID`**: Formerly resourceID, resources are Sygma-specific unique identifiers that define a token or an asset on a domain. It is crucial for asset tracking and management in cross-chain interactions. There can be different types, such as `fungible`, `nonfungible`, `semifungible`, `permissionlessgeneric`, etc.

- **`caip19`**: Representing [caip19](https://chainagnostic.org/CAIPs/caip-19) compatible resource identifier

- **`Asset`**: Refers to any token or digital resource managed on a blockchain within the Substrate framework. It can be native or bridged from another chain.

- **`Handler` Types**: Describes the different types of handlers (e.g., `erc20`, `permissionlessGeneric`, etc.) available to a domain, and their roles in processing specific types of transactions or interactions within the network. For more on handlers, please see [Handlers](../02-sygma-protocol/04-Supported-Ecosystems/01-evm.md#handlers).

- **`feeHandlers`**: Describes the fee strategies available on each domain. For more on fees, please see [Fees](../02-sygma-protocol/05-Fees/01-Fee-intro.md).

<!-- - **`blockConfirmations`**: Under MPC relay, the block confirmations before an accepted attestation can be configured -->

