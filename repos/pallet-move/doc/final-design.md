# Introduction
This article discusses the pallet-move functionality.

- [Pallet API](#pallet-api)
  - [Extrinsics](#extrinsics)
  - [RPC](#rpc)
- [Design architecture](#design-architecture)
- [Multi Signer Script Execution](#multi-signer-script-execution)


# Pallet API
[smove] tool is used to build modules and bundles and to create script transactions (with given script arguments). The output from the `smove` tool in the Move projects is the main input for the below extrinsics.

MoveVM scripts and modules are not allowed to run forever - therefore, the `gas` value is used for that purpose. The provided `gas` gets converted to `Weight`, a mechanism that prevents extriniscs from running forever.
The script execution is an atomic operation - if the provided gas is insufficient, the MoveVM within the pallet will reject the script, and no changes shall occur - but the user will pay for the used weight anyway. So, using the gas estimation RPC method is well recommended.

Scripts and modules have limited access to the balance transfer functionality via the `cheque_limit` parameter - the maximum amount of balance the account scripts can transfer from the signer of the extrinsic.

## Extrinsics

```rust
    /// Execute Move script transaction sent by the user.
    #[pallet::call_index(0)]
    #[pallet::weight(T::WeightInfo::execute(*gas_limit))]
    pub fn execute(
        origin: OriginFor<T>,
        transaction_bc: Vec<u8>,
        gas_limit: u32,
        cheque_limit: BalanceOf<T>,
    ) -> DispatchResultWithPostInfo;
```

```rust
    /// Publish a Move module sent by the user.
    /// Module is published under its sender's address.
    #[pallet::call_index(1)]
    #[pallet::weight(T::WeightInfo::publish_module(*gas_limit))]
    pub fn publish_module(
        origin: OriginFor<T>,
        bytecode: Vec<u8>,
        gas_limit: u32,
    ) -> DispatchResultWithPostInfo;
```

```rust
    /// Publish a Move bundle sent by the user.
    ///
    /// Bundle is just a set of multiple modules.
    #[pallet::call_index(2)]
    #[pallet::weight(T::WeightInfo::publish_module_bundle(*gas_limit))]
    pub fn publish_module_bundle(
        origin: OriginFor<T>,
        bundle: Vec<u8>,
        gas_limit: u32,
    ) -> DispatchResultWithPostInfo;
```

```rust
    /// Publish a standard library bundle, e.g. Move-Stdlib or Substrate-Stdlib. Sudo user only.
    ///
    /// All standard libraries are published at their default address 0x1.
    #[pallet::call_index(3)]
    #[pallet::weight(T::WeightInfo::update_stdlib_bundle())]
    pub fn update_stdlib_bundle(
        origin: OriginFor<T>,
        stdlib: Vec<u8>,
    ) -> DispatchResultWithPostInfo;
```

## RPC

To quickly access these RPC methods above, it is recommended to use `smove node rpc` set of subcommands.

### Method `mvm_estimateGasPublishModule`
Estimate gas and weight cost for publishing a module.

**Parameters**

`account: AccountId` - Account ID which is publishing the module.

`bytecode: Vec<u8>` - Module bytecode.

`at: Option<BlockHash>` - Optional block.

----------------------------------------------------------------

### Method `mvm_estimateGasPublishBundle`
Estimate gas and weight cost for publishing a bundle.

**Parameters**

`account: AccountId` - Account ID which is publishing the module.

`bytecode: Vec<u8>` - Module bytecode.

`at: Option<BlockHash>` - Optional block.

----------------------------------------------------------------

### Method `mvm_estimateGasExecuteScript`
Estimate gas and weight cost for executing a Move script.

**Parameters**

`transaction: Vec<u8>` - Script transaction bytecode.

`at: Option<BlockHash>` - Optional block.

----------------------------------------------------------------

### Method `mvm_getResource`
Get resource from within the MoveVM storage on chain.

**Parameters**

`account: AccountId` - Account ID which is publishing the module.

`tag: Vec<u8>` - Byte representation of the given resource.

`at: Option<BlockHash>` - Optional block.

----------------------------------------------------------------

### Method `mvm_getModuleABI`
Get module ABI using account address.

**Parameters**

`address: &str` - Account address which owns the module.

`name: &str` - Name of the module.

`at: Option<BlockHash>` - Optional block.

----------------------------------------------------------------

### Method `mvm_getModule`
Get module binary using account address.

**Parameters**

`address: &str` - Account address which owns the module.

`name: &str` - Name of the module.

`at: Option<BlockHash>` - Optional block.

----------------------------------------------------------------

# Design Architecture

The main parts are:
- A pallet hosting the MoveVM _(this repo)_.
- The `no-std` MoveVM fork adapted for the Substrate framework.
  - It can be found inside the _[substrate-move][substrate-move]/language/_ directory.
- The backend layer which is an interface between the MoveVM and the pallet.
  - It is located inside the _[substrate-move][substrate-move]/_ directory.
- [smove][smove] tool, which is necessary for the compilation of Move source code.

How it works under the hood is shown in a simple UML diagram below:

| ![uml-pallet-move-full-architecture-m2.png](./assets/uml-pallet-move-full-architecture-m2.png) |
|:--:|
| *Move pallet architecture* |

[smove]: https://github.com/eigerco/smove
[substrate-move]: https://github.com/eigerco/substrate-move

# Multi Signer Script Execution

Executing Move scripts with multiple signers works basically the same as for a single signer.
When the first signer executes a script transaction with multiple signers, the Move pallet will create a multi-signer execution request in the storage.
Each following signer will execute the same script transaction with its individual cheque limit for the planned script execution.
Pallet move will store the state of each signature and each cheque limit and keep track if all users have signed or not.
After each signer has signed by calling the exact same extrinsic call with the same script transaction, the Move pallet will execute the script.

**Differences to single signer scripts:**
- The cheque limit (tokens) of each signer will be locked on their accounts until the request gets finally executed or deleted.
- Except for the final signer, the event `SignedMultisigScript` will be emitted instead of `ExecuteCalled`.
- When all signers have signed, the script will be executed, the tokens be unlocked, and balances applied according to the Move script.
- Every user needs to sign the script within a certain time limit; otherwise, the request will expire, which means it will be removed automatically after a certain amount of time (as defined by the blockchain developer).
- If a multi-signature request expires, then all previous signatures are dropped in vain. If some user then reinitiates the request, all signers need to provide their signature again.
- The point of time of the first signature defines the expiration timeout for that multi-signature script. New signatures for that multi-signer script cannot extend the time limit.
- If all signatures are collected and then the script execution fails (e.g. because of insufficient cheque amount), no change will take place in the MoveVM storage / balance, nor the previous signatures will be dropped. The only way to re-execute the script successfully is to find a signer who provided insufficient `cheque_limit` and ask that user to re-sign the script. Only then the final signer can execute the script successfully.
- The signer order doesn't matter (it is independent of the order of the script function arguments).
- If the script function argument list has a signer in multiple places in the argument list, this signer (user) has to sign the script only once.
- Only the last signer must provide the `gas_limit` value necessary for execution within the MoveVM. All previous signers can set the `gas_limit` value to zero since the script won't start\the execution until all signatures are collected.

We recommend you review our quick multi-signer [tutorial](tutorial-multi-signer.md) for more practical info.
