---
slug: /tailoredsecurity/spectre/intro
id: tailoredsecurity-spectre-intro
title: Introduction To Spectre
description: The following details how Spectre is utilized by Sygma.
draft: false
---

:::info
The following details Sygma's Spectre verification system utilizing zero-knowledge (zk) proofs of consensus between EVM domains. Spectre was developed by ChainSafe as a set of zk cryptographic provers, arithmetic circuits, and contracts. Protocol Sygma integrates Spectre, along with custom infrastructure built to utilize Spectre, to provide zk verification. For ease of definition, this verification system will be generally referred to as "Spectre", but there are distinctions between components built for Sygma and cryptographic components built for Spectre.
:::

# Introduction To Sygma's Spectre

Sygma's **zero knowledge (zk) light client coprocessor, Spectre**, offers a trust-minimized, highly secure verification system that brings high speeds and low cost (at scale) in zk verification to EVM-based environments. At a high level, the verification system is implemented via 1) a set of Spectre-specific contracts (or equivalent in other ecosystems), including SNARK verifier contracts; 2) a Spectre-specific relayer + Spectre prover, and 3) a Sygma inclusion prover.

The Spectre _relayer_, upon detection of an onchain event, requests the Spectre _prover_ to generate a zk-SNARK proof of the source chain state root / block header. The relayer then submits this proof onchain to a proxy contract acting as the state root storage. This triggers the executor contract on the destination chain. A separate offchain inclusion prover must then submit Merkle proofs of transaction/state inclusion in the verified + finalized canonical source chain block to the executor contract. If valid, the execution will then be posted to the destination chain, signifying the end of a Spectre-verified cross-chain interaction.

![](<../../../../static/assets/sygma_spectre_flow.png>)

## What Is Spectre?

Spectre implements a blockchain coprocessor to offload intensive computations from a constrained onchain execution layer to a more expansive offchain environment. Its purpose is to produce succinct proofs of [Gasper](https://ethereum.org/en/developers/docs/consensus-mechanisms/pos/gasper/) consensus that can be efficiently verified on a destination chain. 

This model stands in contrast to the prevailing approach where a committee of relayers post a [signature](../02-MPC/06-signing.md) (generated via [MPC](../02-MPC/02-mpc.md)) stating that the majority of them have detected a certain onchain event. It's crucial to note that neither the threshold signature nor its generation process is concerned with specific consensus rules — they merely confirm the signer's assertions. 

In contrast, the coprocessor proofs are tailored to the exact computations being verified. Therefore, no valid proof can exist for a computation that halts, for instance, due to fraudulent inputs. Spectre’s coprocessor proofs are cryptographic arguments of knowledge, or **SNARK**s (Succinct, Non-Interactive Arguments of Knowledge). A critical aspect of Spectre is that it does not rely on trusted intermediaries and makes no probabilistic economic assumptions.

It may be helpful to separate the components native to Spectre and the components built by Sygma in order to utilize Spectre.

Spectre consists of three main components:
- **Spectre Prover**
- **Light-client Circuits**
- **Spectre Contracts**
  - **Verifier contract**
  - **Light client handler contract**

Sygma-specific components:
- **Relayer** 
- **Inclusion prover** 
- **Sygma contracts**

## Spectre Prover

The prover generates a zero-knowledge proof by performing the necessary computations without revealing any information apart from the validity of a statement. This component utilizes a fork of the [Halo2 proving stack](https://github.com/privacy-scaling-explorations/halo2).

Two types of proofs are submitted onchain:
- Step Proofs
  - These are submitted only for the epoch where a cross-chain interaction is detected by a Sygma relayer. However, depending on the implementation, such as in the case of a Mainnet -> Gnosis route, step proofs are generated every epoch (~3 minutes).
- Rotate Proofs
  - These are submitted every 256 epoch (or ~27 hours on Ethereum) in order to follow Altair sync committee rotations.

:::info
A more general definition for `epoch` for different Gasper-based source chains:
- Ethereum mainnet = 12 seconds * 32 slots = ~6.5 minutes 
- Gnosis = 5 seconds * 32 slots = ~3 minutes
:::

## Light-client Circuits

Circuits in the context of ZK proofs refers to the arithmetic circuit that defines the computation being proved. These circuits are a way to describe computations in terms that are suitable for cryptographic proofs. In Spectre, circuits are implemented using the [halo2-lib](https://github.com/axiom-crypto/halo2-lib) circuit development framework, which ensures that the circuits are both efficient and secure. The circuits specify the series of arithmetic operations that the prover must perform to generate a valid proof. For Spectre, these circuits specifically handle operations related to verifying the Ethereum Altair Light Client protocol. This library contains a number of non-trivial optimization tricks, while its readable SDK prevents most of the soundness bugs and improves auditability.

![](<../../../../static/assets/spectre_lightclient_benchmark.png>)

## Verifier Contract

"Verifier Contracts" are smart contracts deployed on a blockchain that verify the consensus proofs submitted by the prover. These contracts contain the logic necessary to validate the inclusion of transactions in a block that was finalized on the source chain - without having to redo the computation. In Spectre, these contracts are auto-generated and can handle proofs related to the consensus rules of the Ethereum blockchain as modified by the [Altair hardfork](https://ethereum.org/en/history/#altair). The verifier contract checks the proof provided by the prover using only a small amount of data and confirms whether the proof is correct, thereby ensuring that the computation was performed accurately and without tampering.