---
slug: /protocol/relayers
id: protocol-relayers
title: Relayers
description: The following details how relayers are defined and managed by Sygma.
---

:::info
The following details how relayers are defined and managed by Sygma.
:::

_Relayers are entities that ensure bridge decentralization and execute asset and message bridging._

At the core of Sygma exists a **set of relayers**. Relayers are offchain middleware agents whose responsibility is to listen and parse for source chain events. Upon verification of canon on the source chain (either through attestation, proof of computation, or optimistic relay), the relayer network posts to the destination network. 

Under the [tailored security](../02-sygma-protocol/02-Tailored-Security/01-index.md) protocol, separate relayers exist for the various verification mechanisms. 

## MPC Relay

For MPC relay, the relayer network is distributed among several legal entities and operate under a trusted federation model. Spreading the relayer set across several legal entities creates a distribution of responsibilities and mitigates risk by disincentivizing relayers in the set to act in an unfair manner.

Each relayer within the set listens to both the source and destination chains that are being bridged by Sygma. Based on events that are emitted, these relayers then execute relevant actions.

This multi-relayer set is responsible for relaying messages from a source network to a destination network. Relayers are operating with private key share and execution happens on the destination network with the [MPC](../02-sygma-protocol/02-Tailored-Security/02-MPC/02-mpc.md) private key.&#x20;

Communication between relayer parties happens in a [p2p](https://en.wikipedia.org/wiki/Peer-to-peer) manner; and participants of [p2p](https://en.wikipedia.org/wiki/Peer-to-peer) and MPC communication are strictly defined by a configuration file, which allows us to prevent [Sybil Attacks](https://en.wikipedia.org/wiki/Sybil\_attack).

For more on our current list of relayer partners or learn how to become one, please see [Becoming A Relayer Partner](../04-integrating-with-sygma/05-relayer-partner.md).

## Spectre

More details coming soon

## Zipline

More details coming soon