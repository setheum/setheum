---
slug: /protocol/fees/Fee-intro
id: protocol-Fee-intro
title: Introduction To Fees
description: The following details how fees are handled by Sygma.
sidebar_position: 1
---

:::info
The following section details how fees are handled by Sygma.
:::

Sygma collects a service fee for two purposes:

1. Cover transaction costs (gas fees) 
2. Protocol revenues

While gas fees are required to execute transactions on the destination chains, protocol revenues guarantee that Sygma can sustain the operational costs of a public utility and remain contributor-owned.

### Fee Strategies

The protocol gives developers granular control over the handling of fees for any resource (ERC-20, ERC-721, GMP) that can be bridged from one network to another. Even though the specific implementation of fees will differ based on chain architecture (e.g. EVM or Substrate), general functionality of fees will work the same across implementations. 

Each resource is assigned a **fee strategy** for every potential destination network (i.e. domain), where the mapping between the fee strategy and the domain outlines all potential bridging routes for a given resource. The fee strategy defines the set of rules on how fees should be charged when executing deposits on the source chain. 

The Sygma protocol allows for two fee strategies, implemented via **fee handlers** (contracts in EVM, pallets in Substrate):
- [**Fixed fee strategy**](02-Fixed-Fee.md)
- [**Percentage-based fee strategy**](04-Percentage-Based-Fee.md)

![](../../../static/assets/fee-router-general.png)

These fee handlers are mapped between `sygmaResourceID` and the handler address. This logic can be found in the `Bridge.sol` mapping called `_resourceIDToHandlerAddress`:

```solidity
mapping(bytes32 => address) public _resourceIDToHandlerAddress;

function adminSetResource(address handlerAddress, bytes32 resourceID, address contractAddress, bytes calldata args) external onlyAllowed {
    _resourceIDToHandlerAddress[resourceID] = handlerAddress;
    IHandler handler = IHandler(handlerAddress);
    handler.setResource(resourceID, contractAddress, args);
}
```

In Substrate, the `FeeHandlersRouter` handles the configuration of fee routes. The `adminSetResourceHandler` function is used to set up routes for specific `sygmaResourceID`s and destination domains:

```rs
decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        #[weight = 10_000]
        pub fn admin_set_resource_handler(origin, destination_domain: u32, resource_id: Vec<u8>, handler_id: u32) -> DispatchResult {
            ensure_root(origin)?;
            <ResourceHandlers>::insert((destination_domain, resource_id), handler_id);
            Ok(())
        }
```

### Fee Collection

Fees are usually collected in the source (i.e. native) token, but can be collected in any token using a [percentage-based fee strategy](04-Percentage-Based-Fee.md). If you are interested in a custom fee setup, please contact us on [Discord](https://discord.gg/Qdf6GyNB5J) or fill out [this form](https://share.hsforms.com/1K4-T_yaKSp6F06FGk4wsSgnmy2x).