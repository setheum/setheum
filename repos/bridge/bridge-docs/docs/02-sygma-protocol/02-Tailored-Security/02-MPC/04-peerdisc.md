---
slug: /tailoredsecurity/mpc/peerdisc
id: mpc-peerdisc
title: Peer Discovery
description: The following details the peer discovery process for all relayers.
sidebar_position: 3
draft: false
---

:::info
The following details the peer discovery process for all relayers.
:::

Multi-party computation is structured in rounds during which relayers interactively collaborate to produce threshold signatures for asset transfer proposals. Relayers participating in the following operations are required to know the MPC network topology and recognize each other.

Formally these knowledge requirements include:

- Each peer ID
- Each peer address
- Whether the trusted dealer exists
- Trusted dealer address (if exists)
- Rendezvous-point (“meet me here” location)

Relayer nodes are deployed with a configuration in order to serve the above required information. To support the dynamic nature of relayer sets, the `Bridge.sol` contract onchain contains operational parameters that are publicly available with changes that can be actively listened and reacted to. 

## Flow

  ![](<../../../../static/assets/peerdisc_flow.png>)