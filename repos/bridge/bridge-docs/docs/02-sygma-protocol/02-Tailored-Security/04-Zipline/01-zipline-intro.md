---
slug: /tailoredsecurity/zipline/intro
id: tailoredsecurity-zipline-intro
title: Introduction To Zipline
description: The following details how Zipline is utilized by Sygma.
draft: false
---

:::info
The following details Sygma's Zipline (Optimistic fraud proof) verification system.
:::

# Introduction To Zipline

**Zipline** is a gas-efficient, optimistic, fraud-proof verification system, which _optimistically_ assumes transaction validity and offloads dispute verification to external challengers. It is the third verification system in Sygma's Tailored Security protocol stack. It provides a high speed, low cost, and highly secure verification mechanism - when immediate finality is not required. 

## Cross-chain Fault Proofs

The Zipline protocol allows anyone to submit updates, provided that they supply a bond (a security deposit). These updates are subject to a challenge period (7 days), during which anyone can dispute their validity. If a dispute arises, a game begins to identify where potential fraud may have occurred.

This process uses a technique called [bisection](https://en.wikipedia.org/wiki/Bisection_method?ref=blog.buildwithsygma.com) of the [execution trace](https://www.risczero.com/docs/explainers/proof-system/what_is_a_trace?ref=blog.buildwithsygma.com) to pinpoint the suspicious instruction, which the chain then executes to determine if there was fraud.

Zipline functions similarly to optimistic rollups like Optimism and Arbitrum with one key difference: instead of processing and posting transactions, it verifies the _finality_ of another chain.

## Design Overview 

Zipline’s implementation is inspired by [Optimism’s Cannon stack](https://docs.optimism.io/stack/protocol/fault-proofs/cannon). It has been adopted for cross-chain interoperability by focusing on the correctness of finality attestations verification — the stage where a block or a set of transactions has been confirmed as final and cannot be reverted under normal network conditions. Zipline allows anyone to submit updates, provided that they supply a bond (economic security). These updates are subject to a challenge period (7 days), during which anyone can dispute their validity. If a dispute arises, a process of comparing two results (a bisection game) begins, verifying and pinpointing the suspicious instruction. If a challenge is unsuccessful, the bond is forfeited.

Zipline functions similar to fault-proof-based rollups like Optimism and Arbitrum, which assume transactions are correct unless proven otherwise. However, rather than processing transactions, Zipline verifies the finality on the destination chain. By doing so, it effectively acts as a verifier or auditor of the destination chain’s finality protocol, checking for correctness and integrity, enabling the path for new and efficient cross-chain use cases for further interoperability.

## Economic Security

While Zipline accepts permissionless proposals for candidate finalized blocks, it utilizes economic security as an additional layer of defense. Each proposal must be accompanied by a bond, which acts as a financial incentive to align a participant’s behavior with the network’s goals and deter any malicious intentions. If a proposal goes unchallenged within the dispute period, the proposal is
considered final, and the bond is returned.

In the event an observer believes a proposal to be fraudulent, they are eligible to submit a fraud proof on the destination chain within the dispute period to challenge the validity of the transaction. In the context of a cross-chain transaction, this could be a claim that assets on the source chain were not correctly locked or burned prior to bridging to the destination chain. In all optimistic implementations, it is guaranteed that the honest party will prevail. In Zipline’s implementation, a
proven fraud proof will revert the block root and slash the bond of the malicious proposer, and award it to the challenger.