---
id: glossary
slug: /resources/glossary
title: Glossary
description: The following section details important glossary terms related to Sygma
---

:::info 
The following section is broken down into grouped sections. It follows a sequential ordering so that terminologies and concepts might build off each other.
:::

### Table Of Contents
- [Foundational Language](#foundational-language)
- [Cross-chain Language](#cross-chain-language)
- [Sygma Language](#sygma-language)
- [Tailored Security](#tailored-security)

## Foundational Language

**Bridge**: A connection between two blockchain networks, allowing the transfer of data, tokens, or other assets from one chain to another.

**Network**: Refers to the entirety of a blockchain ecosystem, whether it is a layer 1 (L1) or layer 2 (L2) chain, including nodes, protocols, and technologies, e.g. Ethereum, Polygon, Base, Gnosis, Polkadot, Bitcoin, Cronos, Cosmos, etc. 

**Cross-chain**: Denotes an action that occurs across multiple blockchains.

**Interoperability**: The ability for different blockchains to transmit data between each other.

**Source (origin) chain**: Refers to the initial blockchain network from which a transaction, asset, or data originates before being transferred to another chain.

**Destination (target) chain**: The network to which a transaction, asset, or data is sent from another blockchain, usually in the context of cross-chain transfers or communication.

**Route**: The bidirectional path a transaction or message can take as it moves between networks or blockchains in a cross-chain operation. A route may involve intermediary chains other than the source and destination.

**Ethereum Virtual Machine (EVM)**: A computation engine that acts like a decentralized computer, executing smart contracts on the Ethereum network.

**Substrate**: A modular framework for building blockchains (e.g. Polkadot, Kusama), enabling developers to create purpose-built chains tailored to specific applications (e.g. Phala, Astar).

**Pallets**: Components in the Substrate framework that encapsulate specific blockchain functionalities, easily integrated into a Substrate-based blockchain. This is roughly equivalent to a smart contract in EVM-based environments.

**Relay Chain**: The central chain of a parachain network like Polkadot. It provides security and interoperability for all connected parachains. 

**Parachains**: Individual blockchains that run in parallel within the Polkadot or Kusama ecosystem. They connect to the main relay chain to benefit from its security and interoperability features.

**Cross-consensus message format (XCM)**: A messaging format and language used to communicate between Substrate parachains.

---

## Cross-chain Language

**Native interoperability**: The ability of a digital asset, like an ERC-20 token, to be transferred across different blockchain networks and operate as the native asset of the receiving ecosystem, without being wrapped or converted into a synthetic representation. This ensures that the asset retains its original properties and functions directly within the new environment’s protocol.

**Relayers**: Entities in blockchain systems that relay information from one blockchain to another. Instead of operating full nodes, a relayer operator is listening to events on a source or target chain and sending packets of data to and from different blockchains on behalf of the blockchain’s users. 

**Relayer network**: A collection of nodes that facilitate cross-chain communication by transmitting data and transactions between different blockchain networks, using various security mechanisms to ensure data integrity and consistency between each other.

**Burn-and-mint**: A process in blockchain where tokens are 'burned' or destroyed in one network and an equivalent number of tokens are 'minted' or created in another network, commonly used in pegged token mechanisms.

**Lock-and-release**: A mechanism in cross-chain transactions where assets are locked in one blockchain and equivalent assets are released in another blockchain.

**Liquidity provider**: In the cross-chain context, an entity or individual that supplies tokenized assets to a liquidity pool. These assets facilitate cross-chain trading and other financial activities.

**Generic message passing**: A method for sending arbitrary data and messages between different blockchains or layers within a blockchain, often used in cross-chain communication.

--- 

## Sygma Language

**ChainBridge-core**: An extensible cross-chain communication protocol designed to be a framework for cross-chain applications. Sygma extends the foundational Chainbridge-core repo to build its interoperability solution.

**Handlers**: Functions or mechanisms, implemented with smart contracts or pallets, that manage or direct specific types of operations or data processing in the context of cross-chain transactions.

**Chain (ChainID)**: A unique identifier for a specific EVM blockchain network, used in transactions to distinguish between different chains and ensure transactions are processed on the correct blockchain. Commonly known `ChainID`'s can be obtained from [Chainlist](https://chainlist.org/).

**Domain (DomainID)**: Serves a similar purpose to `ChainID` but is specific to the Sygma protocol. It uniquely identifies different domains or environments within Sygma's cross-chain communication framework, ensuring that messages and transactions are correctly routed and processed across different blockchains or segments of the network. `ChainID`'s and `DomainID`'s are registered together, so each `DomainID` is associated with a specific `ChainID`. This information is stored in the chain's storage and you can look up the mappings.

**Resource (ResourceID)**: This is a unique identifier for an asset (e.g. ERC-20 or generic message) in Sygma.

---

## Tailored Security 

**Verification system**: The modular components within the Sygma protocol stack that verify a blockchain’s consensus algorithm and state.

**Multi-party computation**: A subfield of cryptography with the goal of creating methods for parties to jointly compute a function over their inputs while keeping those inputs private.

**Threshold signature schemes**: A digital signature scheme where any _t_ or more signers of a group of _n_ signers can produce signatures on behalf of the group. Threshold cryptography is a cryptosystem that protects information by encrypting it and distributing it among a cluster of fault-tolerant computers. The message is encrypted using a public key, and the corresponding private key is shared amongst the participating parties. With a threshold cryptosystem, in order to decrypt an encrypted message or to sign a message, several parties (more than some threshold number) must cooperate in the decryption or signature protocol.

**MPC-based relay**: A network of nodes that relay information using multi-party computation (MPC), a cryptographic method that allows multiple parties to confirm the consensus of onchain events.

**Coprocessor**: An offchain system that offloads heavy computational steps to verify a blockchain state.

**Zero-knowledge proof (ZKP)**: A ZKP is a cryptographic method that allows one party (the prover) to prove to another party (the verifier) that a statement is true, without revealing any specific information about the statement itself, other than the fact that it’s true.

**Light client**: A type of blockchain client that does not store or process an entire blockchain’s data. However, it can rely on efficient mechanisms to verify that certain transaction have occurred or that a specific state is correct.

**Optimistic Fraud proof**: Proof that can be submitted by a network participant to challenge and prove the incorrectness of a fraudulent transaction.

**Challengers**: Entities or individuals in a blockchain system that can dispute the validity of a transaction, especially in optimistic rollup systems. If a transaction is found to be invalid, the challenger may receive a reward.