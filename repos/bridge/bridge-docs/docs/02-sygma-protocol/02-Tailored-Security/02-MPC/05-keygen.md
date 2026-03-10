---
slug: /tailoredsecurity/mpc/keygen
id: mpc-keygen
title: Key Generation
description: The following details the key generation process for all relayers.
sidebar_position: 4
draft: false
---

:::info
The following details the key generation process for all relayers.
:::

Recall from the previous section that the private key used for signing transactions is distributed and never held in full by any one entity. In order to facilitate this, all *n* relayers must collaborate together to generate the public and private key shares (sic keygen). This operation should be considered the most computationally expensive and thus recommended to be executed once and repeated only as a recovery measure after critical network segmentation (where relayer entities drop off or no longer exist).

:::info
The private key that gets generated is done in parts and never held in one place all together.
:::

Ultimately, we can look on the key generation (i.e. keygen) as the access control mechanism for Sygma. Meaning that only those relayers who had participated at this initial ceremony are allowed to contribute their computation within other flows.

## Flow

1. Admin calls `StartKeygen()` of `Bridge.sol` contract
2. Event `KeygenRequired` is emitted by `Bridge.sol` contract
3. Relayers that listen to events initiate `Keygen` protocol
  ![](<../../../../static/assets/keygen_flow.png>)
4. Relayers observe chain state and listen to events that signal public key submission
5. If public key *y* hasn’t been submitted yet, any one relayer can transact it onchain signaling the finalization of the protocol

:::warning
To obtain the private key in full would require the corruption of greater than the threshold number of relayers in the signature scheme. Given *n* relayers and threshold *t*, if less than *n-t+1* are compromised, it is still safe. If greater than *n-t+1* but less than *t+1*, execution can be temporarily halted. But since it is an identifiable scheme, the rest of the relayer set can respond appropriately.
:::