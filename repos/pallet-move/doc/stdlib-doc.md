# Move Standard Library In Pallet-Move

The standard library provides a set of common modules that should be available to every Move developer from the genesis block.
All modules within the standard library are published under the special account address `0x01` in the Move storage.

The standard library is contained in two repositories:
 - [`move-stdlib`][move-stdlib] - commonly known set of standard library modules that can be found across all Move compatible ecosystems.
 - [`substrate-stdlib`][substrate-stdlib] - an extension library to the standard library which contains Substrate-specific modules or any module that might be required for specific pallet-move use cases.

If the pallet users want to extend the standard library, we recommend adding new modules to the `subtrate-stdlib` repository to keep the original `move-stdlib` small and simple.

During the design of this customizable pallet, we abstained from adding too many new modules in order to keep the minimal working set of Move modules.
Any new modules can be easily added once the end goal for the pallet user is clearly defined.

In the following chapters, one can find a set of instructions that can be used to update the standard library when there is a clear need to do so.

Before we delve into the next two chapters, we should mention the special kind of functions called [native functions][native-fn].
These functions don't have a regular function body specified in Move source files like normal functions have, but instead, their implementation is integrated within the MoveVM itself.
Native functions are only allowed in standard libraries, as updating them requires updating the MoveVM source code itself.

## Move Modules Updates

Adding new modules to the standard library is pretty simple.
Add a module to the wanted repository and then build it as a bundle with `smove bundle` and also generate doc files with `smove docgen`.

If the standard library needs to be updated in post-genesis block settings, the `update_stdlib_bundle` extrinsic can achieve this.
Check usage instructions in the [tech guide](./tech_guide.md#update-standard-libraries).
Note: This extrinsic can be used only by the root user.

A simple example can be found in this [pull request](https://github.com/eigerco/move-stdlib/pull/5).

## Native Function Updates

Adding new native functions to the standard library requires the implementation of a function body in Rust within the MoveVM code.

You can use these PRs as an example of how that can be done:
 - in [`substrate-move`][substrate-move]: [feat: add native functions for substrate_hash module](https://github.com/eigerco/substrate-move/pull/83)
 - in [`substrate-stdlib`][substrate-stdlib]: [feat: add new hash module with native functions](https://github.com/eigerco/substrate-stdlib/pull/4)

Whenever native functions are added, [smove][smove] needs to update dependencies so it fetches the latest MoveVM changes and then a new version of `smove` should be published.

Hopefully, pallet users won't need to add new native functions after the genesis block, otherwise the node's runtime will require an update containing the new version of the MoveVM instance with the latest code that includes added native functions.

[move-stdlib]: https://github.com/eigerco/move-stdlib
[substrate-move]: https://github.com/eigerco/substrate-move
[substrate-stdlib]: https://github.com/eigerco/substrate-stdlib
[native-fn]: https://move-language.github.io/move/functions.html?highlight=native#native-functions
[smove]: https://github.com/eigerco/smove
