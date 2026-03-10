---
slug: /tailoredsecurity/mpc/signing
id: mpc-signing
title: Proposal Signing
description: The following details the signing of proposals by relayers.
sidebar_position: 5
draft: false
---

:::info
The following details the signing of proposals by relayers.
:::

Signing in this context is an operation where relayers collaborate to produce a signature of a given proposal and further submit it onchain. The threshold properties and MPC architecture are what allow one to do that in a secure way without directly relying on the expensive security provided by the destination chains.

After the key generation, each of *n* relayers has a key share *x* that will allow them to participate in the signing protocol. However, only a subset of *t* relayers is required to construct a full signature.

## Flow 

1. User calls `deposit` method of `Bridge.sol` contract
2. Event `Deposit` is emitted
3. Relayers observe event `Deposit`, formulate message *M* and initiate `Keysign` protocol
    ![](../../../../static/assets/keysign_flow.png)
4. Relayers observe chain state and listen to events of signature submission for the current proposal
5. If signature *σ* hasn’t been submitted yet, any one relayer can transact it onchain signaling the finalization of the protocol

