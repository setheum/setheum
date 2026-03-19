[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE)

# MoveVM for Substrate

This repository is part of the [pallet-move] project, which enables users to write smart contracts in Substrate-based blockchains with the Move language.
This is a modified MoveVM fork for the use of MoveVM in the [pallet-move] Substrate repo.

## Requirements

`smove` is a package manager for Move language in Substrate. Follow the [instructions][smove] to install it.

## Development

For the initial development setup, run the script:
```sh
./scripts/dev_setup.sh -ypt
```

### Design

[pallet-move] uses the `move-vm-backend` to interact with the MoveVM.

The integral part of MoveVM functionality still lies within the `language` directory and contains only the necessary modifications which make the MoveVM operable within the Substrate framework.

### Testing

To run tests for the MoveVM implementation, execute:
```sh
cargo test
```

To run tests for the `move-vm-backend` implementation, execute:
```sh
cargo test -p move-vm-backend --features build-move-projects-for-test # the `backend` main crate
cargo test -p move-vm-backend-common --features build-move-projects-for-test # helper crate for interaction with smove and pallet-move
cargo test -p move-vm-support # helper crate for interaction with language directory
```
_Note: the feature flag `build-move-projects-for-test` needs to be provided only once in order to build all the necessary `move-vm-backend/tests/assets/move-projects/` projects for test purposes (with the `smove-build-all.sh` script). Also, the feature flag needs to be provided whenever any of those Move projects are modified._

## Open / Planned Points
- clean clippy warnings for the latest rustc compiler
- improve get_resource API (input params and the output value)

## See also

- [pallet-move] - Main repo containing the Move pallet.
- [move-stdlib] - Provides elementary Move functions in Move smart contracts. 
- [smove] - Handles the gas estimation, the serialization of script and module transactions, and the inspection of the module's ABIs.
- [substrate-stdlib] - Provides elementary Substrate functions in Move smart contracts.

## License

Move is licensed as [Apache 2.0](https://github.com/move-language/move/blob/main/LICENSE).

## About [Eiger](https://www.eiger.co)

We are engineers. We contribute to various ecosystems by building low level implementations and core components. We believe in Move and in Polkadot and wanted to bring them together. Read more about this project on [our blog](https://www.eiger.co/blog/eiger-brings-move-to-polkadot).

Contact us at hello@eiger.co
Follow us on [X/Twitter](https://x.com/eiger_co)


[move-stdlib]: https://github.com/eigerco/move-stdlib
[pallet-move]: https://github.com/eigerco/pallet-move
[smove]: https://github.com/eigerco/smove
[substrate-stdlib]: https://github.com/eigerco/substrate-stdlib
