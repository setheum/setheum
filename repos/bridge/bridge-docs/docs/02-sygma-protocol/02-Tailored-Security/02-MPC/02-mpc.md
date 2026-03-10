---
slug: /tailoredsecurity/mpc/intro
id: tailoredsecurity-mpc-intro
title: Multi-party Computation (MPC)
description: The following details how MPC is utilized by Sygma.
---

:::info
The following details Sygma's multi-party computation verification system.
:::

# Introduction To Sygma's Multi-party Computation (MPC)

Sygma's Tailored Security system begins with its base **multi-party computation (MPC)** verification. For routes (i.e. source to destination chain) integrated with MPC, users and developers can enjoy a low cost, high speed, and secure service. At a high level, the verification system is implemented via 1) a set of MPC-specific contracts (or equivalent in other ecosystems) and 2) an MPC-specific relayer network.

Onchain events are generated as a result of a cross-chain interaction (through a Sygma-integrated dApp). The MPC relayer network listens for and parses these cross-chain events. Multi-party computation methodologies are then used to determine whether these events are canonical; i.e. can it be verified that these events *actually* happened? The MPC relayer network will then make attestations and post these events to the destination chain, signifying the end of an MPC-verified cross-chain interaction. 

## What Is Multi-party Computation?

MPC represents a powerful next step in digital asset security because it eliminates the risk of a single point of compromise.

Instead of relying on [Multisigs](https://en.wikipedia.org/wiki/Multisignature) or other, older ways of key management that either expose [relayer](../../03-relayers.md) identities or introduce easily exploitable single points-of-failure, relayers for Sygma run a secure MPC ceremony each time a user wishes to bridge funds or transfer arbitrary data. In this way, MPC enables multiple parties to carry out a distributed computation on their secret inputs without revealing anything but the output.&#x20;

MPC was introduced as a solution for the **[Two Billionaires Problem](https://en.wikipedia.org/wiki/Yao%27s_Millionaires%27_problem)** (Bob and Alice; how to decide who is richer without showing their exact funds, a specific problem which is a Boolean predicate).

The multi-party computation (MPC) model that Sygma employs includes a number of trusted relayer nodes operating under [a trusted federation](https://blog.chainsafe.io/bridges-in-crypto-space-12e158f5fd1e). These trusted relayer nodes are run by reputable entities in the web3 space.

For a detailed research piece, please see [Multi-Party Computation: The Next Generation of Crypto Security](https://blog.buildwithsygma.com/multi-party-computation/) from our blog.
