# substrate-stdlib for pallet-move

This repository is part of the [pallet-move] project, which enables users to write smart contracts in Substrate-based blockchains with the Move language.
This standard library can be included to provide elementary Substrate functions in Move smart contracts.

Currently, it contains the module:
- **balance**: Supports the transfer of balances on the blockchain and checking current balances on accounts. Find more details in the [module documentation][mod-doc-balance].
- **substrate_hash**: Set of useful hash functions. Find more details in the [module documentation][mod-doc-hash].

## See also

- [pallet-move] - Main repo containing the Move pallet.
- [move-stdlib] - Provides elementary Move functions in Move smart contracts. 
- [substrate-move] - A modified MoveVM fork for the use of MoveVM in the pallet-move repo.
- [smove] - Handles the gas estimation, the serialization of script and module transactions, and the inspection of the module's ABIs.

## License

substrate-stdlib is licensed as [MIT](LICENSE).

## About [Eiger](https://www.eiger.co)

We are engineers. We contribute to various ecosystems by building low level implementations and core components. We believe in Move and in Polkadot and wanted to bring them together. Read more about this project on [our blog](https://www.eiger.co/blog/eiger-brings-move-to-polkadot).

Contact us at hello@eiger.co
Follow us on [X/Twitter](https://x.com/eiger_co)


[mod-doc-balance]: doc/balance.md
[mod-doc-hash]: doc/substrate_hash.md
[move-stdlib]: https://github.com/eigerco/move-stdlib
[pallet-move]: https://github.com/eigerco/pallet-move
[smove]: https://github.com/eigerco/smove
[substrate-move]: https://github.com/eigerco/substrate-move
