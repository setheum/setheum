---
slug: /tailoredsecurity/mpc/tss
id: mpc-tss
title: Threshold Signature Scheme
description: The following details how Sygma addresses security concerns relating to various signature schemes.
sidebar_position: 2
draft: false
---

:::info
  The following details how Sygma addresses security concerns relating to
  various signature schemes.
:::

## Threshold Signature Schemes (TSS)

**Threshold Signature Schemes (TSS)** is an area of [MPC](../../../02-sygma-protocol/02-Tailored-Security/02-MPC/02-mpc.md) that is particularly useful for the crypto domain as it facilitates the distribution of a private key to multiple parties, introducing redundancy for assets management security.&#x20;

In other words, it enables a set of parties to perform certain cryptographic operations, like signing transactions, while none of them hold a full private key. Instead, the key is split across the parties, and it can only be used when a subset of those parties — the size of which is larger than a certain threshold — combine their key shares.

### An Example

Imagine you have a secret key sk and [a special algorithm](../../../02-sygma-protocol/02-Tailored-Security/02-MPC/05-keygen.md) that can divide this key into *n* pieces such that *[sk_i] = share_key(pk, n, t)*. Imagine now you want to [sign a transaction](../../../02-sygma-protocol/02-Tailored-Security/02-MPC/06-signing.md) *m*, so you apply a similar algorithm to get partial signatures *[s_i] = sign(m, [sk_i])*. Now, to reconstruct a valid signature, you would simply sum all partial signatures together *s = s_0 + s_1 + … + s_i* and call it a day.

You might’ve also noticed a third argument *t* when we shared our key. Although the key is shared between *n* parties, we only need a threshold number of them to actually sign something. This is akin to a multisig scheme, which interestingly is just an emulation of threshold signatures using a high-level smart contract language like Solidity.

Of course, multisigs come with a cost where one would pay miners to process each call to the multisig contract. Conversely, threshold signatures are processed off-chain, and only a single compact ECDSA signature needs to be transacted onchain once. Furthermore, such a signature won’t leak anything about its signers, which secures them from targeted attacks and is great for privacy.

When discussing security, MPC algorithms generally provide guarantees based on the threshold number of corrupted parties a system can tolerate. This places TSS in a unique position, as such schemes present the control of their robustness directly in the developer’s hands. Furthermore, this allows it to withstand even the dishonest majority — an extreme state where adversaries can corrupt all but one participant.

### MPC Usage In Sygma

While designing Sygma, we were convinced that having MPC for relayer communication would not only strengthen the overall security of the system but would also significantly reduce fees for the users making the experience much more appealing.

For a detailed research piece, please see [Multi-Party Computation: The Next Generation of Crypto Security](https://blog.buildwithsygma.com/multi-party-computation/) from our blog.

For more on Sygma's approach to security, please see [A Defense-in-Depth Approach: Notes on Sygma Security](https://blog.buildwithsygma.com/sygma-security/) from our blog. 

