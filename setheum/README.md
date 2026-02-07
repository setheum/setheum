بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

<p align="center">
  <img src="./media/SetheumLabel.jpg" style="width:1300px" />
</p>

# Setheum Blockchain

Core implementation of the Setheum Network, a Substrate-based blockchain.

## Directory Structure

- `node/`: The node implementation.
- `runtime/`: The blockchain runtime.
- `modules/`: Custom pallets (modules) for Setheum.
- `primitives/`: Shared types and utilities.
- `clisee/`: Command-line interface for Setheum.
- `setheum-client/`: Client-side libraries.
- `orml/`: Open Runtime Module Library components.

## Building

```bash
cargo build --release
```

## Running

```bash
./target/release/setheum-node --dev
```

## License

The code in this repository is licensed under the [GNU GPL Version 3 License](./LICENSE)

Unless you explicitly state otherwise, any contribution that you submit to this repo shall be licensed as above (as defined in the [GNU GPL-3 Version 3.0 or later WITH Classpath-exception-2.0](./LICENSE)), without any additional terms or conditions.
