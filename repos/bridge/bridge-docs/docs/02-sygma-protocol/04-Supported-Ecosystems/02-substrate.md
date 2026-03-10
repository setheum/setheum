---
slug: /protocol/ecosystem/substrate
id: protocol-substrate
title: Substrate Support
description: The following details Substrate support on Sygma.
---

:::info
The following section details Sygma's compatibility with Polkadot and Kusama parachains via its Substrate pallet support. 
:::

# Introduction To Substrate Support

A unique feature of the Sygma protocol is its ability to provide interoperability between EVM-based and Substrate-based blockchains. This includes those [parachains](https://wiki.polkadot.network/docs/learn-parachains) deployed under Polkadot and Kusama's [shared security model](https://wiki.polkadot.network/docs/learn-architecture).

Similar to how Sygma enables EVM support by invoking various smart contracts, Sygma provides Substrate support via interactions with [custom Sygma pallets](https://github.com/sygmaprotocol/sygma-substrate-pallets). 

:::note
Substrate pallets provide a modular approach to building Substrate-based blockchains and contain runtime logic which can be used, modified, and extended accordingly. 
:::

![](../../../static/assets/Substrate%20__%20EVM.png)

---

### Key Concepts

Sygma's custom pallets contains a multitude of different identifiers with complex interrelationships. These may appear overwhelming at first for EVM developers crossing the chasm, but we will explain the concepts below:

- **MultiLocation**: `MultiLocation` is a cross-consensus message format (XCM) concept used to identify any single entity location that exists within the world of Polkadot consensus. `MultiLocation` always expresses a relative location to the current location. Practically, `MultiLocations` are used to identify places to send XCM messages. In Sygma pallets, it is used to identify the destination when depositing.
- **MultiAsset**: `MultiAsset` is a Substrate type that identifies whether assets are fungible or non-fungible, native or foreign, etc. It is used to handle multiple assets in the Polkadot world. In Sygma, `MultiAsset` is used to identify the asset despite its location and fungibility. 
-  **DomainID & ChainID**: In the context of the Sygma pallets, a `DomainID` is a unique identifier for a particular domain, and a `ChainID` is a unique identifier for a specific blockchain. These IDs are registered together, so each `DomainID` is associated with a specific `ChainID`. This information is stored in the chain's storage, and you can look up which `ChainID` is associated with a particular `DomainID`.
-  **ResourceID**: This is a unique identifier for an asset in Sygma. It's linked with an XCM asset through a mapping defined in the runtime, which associates an (XCM) `AssetID` with a `ResourceID`. This mapping is known as `ResourcePairs`, as defined later in this section.
-  **AssetID**: This can refer to two different types of `AssetIDs`, one of u32 type and the other of XCM type.

    - *AssetID of u32*: This is an identifier assigned to an asset in a Substrate blockchain. It's displayed in the assets page on the Polkadot JS App and is used when transferring the asset between accounts within the same parachain.

    - *AssetID of XCM*: This is a type of `AssetID` used in the context of XCM (Cross-Chain Message) communication. It's an alias for `XcmAssetID` in Sygma pallets and serves as the asset identifier within the entire Polkadot ecosystem. It's linked with the asset's `MultiLocation`, which is a data structure that describes where the asset is located.
- **SimpleForeignAssetConverter**: This is a structure that binds the u32 `AssetID` with the Asset's `MultiLocation`, establishing a link between the two.

- **ResourcePairs**: This is a mapping defined in the runtime that links the `XcmAssetID` and the `ResourceID`.

Practically speaking, the `MultiLocation` of an asset plays a key role in the relationship between the asset, its different IDs (u32 `AssetID` and `XcmAssetID`), the basic fee associated with the asset, and how the asset is linked with a `ResourceID`.

---

For more on Substrate, please refer to their [docs](https://docs.substrate.io/quick-start/). For more on the Polkadot ecosystem, please refer to their [wiki](https://wiki.polkadot.network/docs/getting-started).

