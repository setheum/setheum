# Gas Handling and Weight Costs in Pallet-Move

The gas unit is an internal processing fee for publishing modules and executing scripts within the MoveVM.

In Substrate-based blockchains, another unit, `Weight`, measures and manages the time it takes to validate a block.
Substrate defines one unit of weight as one picosecond of execution time on reference hardware.

## Gas Handling in Move Layer

From within the MoveVM, there are two different sources for gas costs:
- Cost for script execution,
- Cost for publishing modules and bundles.

Script execution can broken down into:
- Gas cost per bytecode instruction - bytecode gas cost table can be found [here](https://github.com/eigerco/substrate-move/blob/d33762e0ec9ee82ca71d7c5991247eec135666d1/move-vm-backend-common/src/gas_schedule.rs#L39)
- Gas cost per native function call with variable gas cost depending on the input function arguments - native gas cost table can be found [here](https://github.com/eigerco/substrate-move/blob/d33762e0ec9ee82ca71d7c5991247eec135666d1/move-vm-backend-common/src/gas_schedule.rs#L166)

It is possible to modify the table values to control the cost of script execution.
It is recommended that both tables be scaled by the same factor to preserve the proportional cost relationship between native functions and bytecode instructions.
These table values are less dependent on storage access and are more related to raw processing time.

The cost for publishing modules and bundles is independently defined from the script execution costs and it can be controlled by modifying [gas-cost-per-byte constant](https://github.com/eigerco/substrate-move/blob/d33762e0ec9ee82ca71d7c5991247eec135666d1/move-vm-backend-common/src/gas_schedule.rs#L23).
This value was selected during the testing and benchmarks and it is an arbitrary choice specifically for the MoveVM instance running within the Substrate runtime.

All internal MoveVM gas handling costs are defined in the same [gas schedule module](https://github.com/eigerco/substrate-move/blob/main/move-vm-backend-common/src/gas_schedule.rs) so that tweaking any gas-related factor can be done from within that module.

## Extrinsic Weight Cost in Pallet Layer

Three main extrinsics interact with MoveVM, which stores its state within the Substrate storage:
- `publish_module`
- `publish_module_bundle`
- `execute`

All above extrinsics have a `gas_limit` argument which is used as an input to the `MoveVM`.
Since the required minimum `gas_limit` can vary a lot between different extrinisic calls, the extrinsic weight cost formula also heavily depends on the `gas_limit` on top of the fixed base extrinisic cost.
That means that using more than the necessary amount of `gas_limit` leads to an unnecessarily more costly extrinsic.
For that reason, it is recommended to use `smove node rpc` estimation RPC commands which provide data about:
- the minimum required `gas_limit` to execute/publish a given script/module,
- an estimated weight cost for the extrinsic call for the given `gas_limit`.

Previously executed Substrate benchmarks define conversion between gas limit and weight and can be found in the auto-generated file [here](../pallet/src/weights.rs).
To better understand the costs, it's best to use the estimation RPC methods.

> [!NOTE]
> The same amount of `gas_limit` between different extrinsic doesn't necessarily add the equal weight cost.

Future pallet users should monitor and observe gas handling costs once the pallet gets integrated into the actual blockchain and then recalibrate it according to their needs, if necessary, according to the info above.
