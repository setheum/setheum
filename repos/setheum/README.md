بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

<p align="center">
  <img src="./media/SetheumLabel.jpg" style="width:1300px" />
</p>

# Setheum

[![Build](https://github.com/setheum/setheum/actions/workflows/ci.yml/badge.svg)](https://github.com/setheum/setheum/actions/workflows/ci.yml)
[![CodeQL](https://github.com/setheum/setheum/actions/workflows/github-code-scanning/codeql/badge.svg?style=flat-square)](https://github.com/setheum/setheum/actions/workflows/github-code-scanning/codeql)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg?style=flat-square)](./LICENSE-GPL3)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue?logo=apache&style=flat-square)](./LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/License-MIT-green?logo=mit&style=flat-square)](./LICENSE-MIT)

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

The projects in the Setheum blockchain are licensed as follows:

- **Core Blockchain & Runtime**: Licensed under the [GNU GPL Version 3](./LICENSE-GPL3) (GPL-3.0-or-later WITH Classpath-exception-2.0).
- **Client & Tooling**: Certain components are dual-licensed under [Apache License 2.0](./LICENSE-APACHE) or [MIT License](./LICENSE-MIT) at your option:

  - `clique/`
  - `setheum-client/`
  - `aggregator/`
  - `tests/flooder/`
  - `rate-limiter/`

Unless you explicitly state otherwise, any contribution that you submit to this repo shall be licensed as above, without any additional terms or conditions.

