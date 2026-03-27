# setheum-move

This repository is part of the [pallet-move] project, which enables users to write smart contracts in Substrate-based blockchains with the Move language.
It's a package manager for the Move language in Substrate. Handles the gas estimation, the serialization of script and module transactions, and the inspection of the module's ABIs.


## Installation

The only requirement is to have [cargo] installed.

Install `setheum-move` from the source with `cargo` by running the following:
```sh
cargo install --git  https://github.com/eigerco/setheum-move
```


## Build Commands

The following command snippets are taken out of [our pallet-move tutorial][tutorial]. Here, the developer Bob, with the account ID _5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty_, publishes a module named `CarWash`. To get the full impression of `setheum-move`, we recommend going through our tutorial!

### Build / Compile Projects

When in move project's directory:
```sh
setheum-move build
```
or with absolute/relative path:
```sh
setheum-move build -p pallet-move/src/assets/move-projects/car-wash-example
```

### Generate Bundles

```sh
setheum-move bundle
```


## RPC Commands

The assumption is a substrate node with pallet-move integrated running at the local host.

### Estimating Gas for Module Publication

```sh
setheum-move node rpc estimate-gas-publish-module --account-id 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty --module-path build/car-wash-example/bytecode_modules/CarWash.mv
```

### Estimating Gas for Script Execution

```sh
setheum-move node rpc estimate-gas-execute-script -s build/car-wash-example/script_transactions/initial_coin_minting.mvt
```

### Request a Module's ABI

```sh
setheum-move node rpc get-module-abi --address 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty --name CarWash
```

### Create Transactions

```sh
setheum-move create-transaction --compiled-script-path build/car-wash-example/bytecode_scripts/initial_coin_minting.mv --args signer:5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
```


## More Functions

`setheum-move` is a derivative from [move-cli] and was extended by the functions shown above. The tool provides all other commands the original `move-cli` provides, which you can check out.

Use also the integrated help pages to get more information:
```sh
setheum-move --help
```
```sh
setheum-move 0.6.0
Eiger <hello@eiger.co>
CLI frontend for the Move compiler and VM in Substrate

USAGE:
    setheum-move [OPTIONS] <SUBCOMMAND>

OPTIONS:
        --abi
            Generate ABIs for packages

        --arch <ARCHITECTURE>


        --bytecode-version <BYTECODE_VERSION>
            Bytecode version to compile move code

    -d, --dev
            Compile in 'dev' mode. The 'dev-addresses' and 'dev-dependencies' fields will be used if
            this flag is set. This flag is useful for development of packages that expose named
            addresses that are not set to a specific value

        --doc
            Generate documentation for packages

        --fetch-deps-only
            Only fetch dependency repos to MOVE_HOME

        --force
            Force recompilation of all packages

    -h, --help
            Print help information

        --install-dir <INSTALL_DIR>
            Installation directory for compiled artifacts. Defaults to current directory

    -p, --path <PACKAGE_PATH>
            Path to a package which the command should be run with respect to

        --skip-fetch-latest-git-deps
            Skip fetching latest git dependencies

        --test
            Compile in 'test' mode. The 'dev-addresses' and 'dev-dependencies' fields will be used
            along with any code in the 'tests' directory

    -v
            Print additional diagnostics if available

    -V, --version
            Print version information

SUBCOMMANDS:
    build                 Build the package at `path`. If no path is provided defaults to
                              current directory
    bundle                Create a package bundle
    coverage              Inspect test coverage for this package. A previous test run with the
                              `--coverage` flag must have previously been run
    create-transaction    Create a script transaction
    disassemble           Disassemble the Move bytecode pointed to
    docgen                Generate javadoc style documentation for Move packages
    errmap                Generate error map for the package and its dependencies at `path` for
                              use by the Move explanation tool
    experimental          (Experimental) Run static analyses on Move source or bytecode
    help                  Print this message or the help of the given subcommand(s)
    info                  Print address information
    new                   Create a new Move package with name `name` at `path`. If `path` is not
                              provided the package will be created in the directory `name`
    node                  Commands for accessing the node
    prove                 Run the Move Prover on the package at `path`. If no path is provided
                              defaults to current directory. Use `.. prove .. -- <options>` to pass
                              on options to the prover
    sandbox               Execute a sandbox command
    test                  Run Move unit tests in this package
```

## See also

- [pallet-move] - Main repo containing the Move pallet.
- [move-stdlib] - Provides elementary Move functions in Move smart contracts. 
- [substrate-move] - A modified MoveVM fork for the use of MoveVM in the pallet-move repo.
- [substrate-stdlib] - Provides elementary Substrate functions in Move smart contracts.

## About [Eiger](https://www.eiger.co)

We are engineers. We contribute to various ecosystems by building low level implementations and core components. We believe in Move and in Polkadot and wanted to bring them together. Read more about this project on [our blog](https://www.eiger.co/blog/eiger-brings-move-to-polkadot).

Contact us at hello@eiger.co
Follow us on [X/Twitter](https://x.com/eiger_co)


[cargo]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[move-cli]: https://github.com/eigerco/substrate-move/tree/main/language/tools/move-cli
[move-stdlib]: https://github.com/eigerco/move-stdlib
[pallet-move]: https://github.com/eigerco/pallet-move
[substrate-move]: https://github.com/eigerco/substrate-move
[substrate-stdlib]: https://github.com/eigerco/substrate-stdlib
[tutorial]: https://github.com/eigerco/pallet-move/blob/main/doc/tutorial.md
