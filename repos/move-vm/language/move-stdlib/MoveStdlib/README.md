# move-stdlib for pallet-move

This repository is part of the [pallet-move] project, which enables users to write smart contracts in Substrate-based blockchains with the Move language.
This standard library can be included to provide elementary Move functions in Move smart contracts.
To provide elementary pure Substrate functions, have a look at the [substrate-stdlib].

Currently, it contains the modules:
- **ascii**
- **bcs**
- **bit_vector**
- **error**
- **fixed_point32**
- **hash**
- **option**
- **signer**
- **string**
- **type_name**
- **unit_test**
- **vector**
- **acl**


## See also

- [pallet-move] - Main repo containing the Move pallet.
- [substrate-move] - A modified MoveVM fork for the use of MoveVM in the pallet-move repo.
- [smove] - Handles the gas estimation, the serialization of script and module transactions, and the inspection of the module's ABIs.
- [substrate-stdlib] - Provides elementary Substrate functions in Move smart contracts.


## License

Move-stdlib is licensed as [APACHE 2.0](LICENSE).


## About [Eiger](https://www.eiger.co)

We are engineers. We contribute to various ecosystems by building low level implementations and core components. We believe in Move and in Polkadot and wanted to bring them together. Read more about this project on [our blog](https://www.eiger.co/blog/eiger-brings-move-to-polkadot).

Contact us at hello@eiger.co
Follow us on [X/Twitter](https://x.com/eiger_co)


[pallet-move]: https://github.com/eigerco/pallet-move
[smove]: https://github.com/eigerco/smove
[substrate-move]: https://github.com/eigerco/substrate-move
[substrate-stdlib]: https://github.com/eigerco/substrate-stdlib
