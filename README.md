بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

<p align="center">
  <img src="./setheum/media/SetheumLabel.jpg" style="width:1300px" />
</p>

<div align="center">
<br />

[![Build](https://github.com/setheum/setheum/actions/workflows/ci.yml/badge.svg)](https://github.com/setheum/setheum/actions/workflows/ci.yml)
[![CodeQL](https://github.com/setheum/setheum/actions/workflows/github-code-scanning/codeql/badge.svg?style=flat-square)](https://github.com/setheum/setheum/actions/workflows/github-code-scanning/codeql)

<br />

[![Website](https://img.shields.io/badge/Website-gray?logo=web)](https://setheum.com)
[![Twitter URL](https://img.shields.io/twitter/url?style=social&url=https%3A%2F%2Ftwitter.com%2FSetheum)](https://twitter.com/Setheum)
[![Telegram](https://img.shields.io/badge/Telegram-gray?logo=telegram)](https://t.me/SetheumNetwork)
[![Lines of Code](https://img.shields.io/badge/LinesOfCode-gray?logo=LinesOfCode)](https://cloc.info/github.com/setheum/setheum)
</div>

* Decentralized
* Exceptional
* Secure
* Innovative
* Reliable
* Ethical
* Scalable

# Setheum Monorepo

Welcome to the Setheum ecosystem. This repository contains all core components organized for clarity and development efficiency.

## Projects

- **[cargo-sheyth](./cargo-sheyth)**: The CLI tool for Setheum smart contract development.
- **[set-bft](./set-bft)**: The Set-BFT Consensus Engine.
- **[setheum](./setheum)**: The core Setheum blockchain.
- **[setheum-js](./setheum-js)**: JavaScript/TypeScript SDK for interacting with Setheum.
- **[sheyth](./sheyth)**: The Setheum Smart Contract Framework.

## Development

This project uses [mise](https://mise.jdx.dev/) for managing development tools and tasks.

### 1. Install Mise
Follow the [mise installation guide](https://mise.jdx.dev/getting-started.html).

### 2. Setup Tools
Install all required tool versions (Rust, Node.js, Python, Yarn) automatically:
```bash
mise install
```

### 3. Run Tasks
Mise handles all common development tasks:

- **Build everything**: `mise run build`
- **Run tests**: `mise run test`
- **Format code**: `mise run fmt`
- **Apply headers**: `mise run headers`
- **Clean artifacts**: `mise run clean`

A [Makefile](./Makefile) is also provided as a proxy for these commands, so you can still use `make build`, `make test`, etc.

## License

The projects in this monorepo are licensed as follows:

- **Core Blockchain & Runtime**: Licensed under the [GNU GPL Version 3](./LICENSE-GPL3.md) (GPL-3.0-or-later WITH Classpath-exception-2.0). This includes the `setheum` node, runtime, and custom modules.
- **Frameworks & Tools**: Licensed under either [Apache License 2.0](./LICENSE-APACHE.md) or [MIT License](./LICENSE-MIT.md) at your option. This includes:
  - `setheum-js`
  - `sheyth`
  - `cargo-sheyth`
  - `set-bft`
  - `setheum/clique`
  - `setheum/setheum-client`
  - `setheum/aggregator`
  - `setheum/tests/flooder`
  - `setheum/rate-limiter`

Unless you explicitly state otherwise, any contribution that you submit to this repo shall be licensed as above, without any additional terms or conditions.

