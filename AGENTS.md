# Setheum Monorepo — AGENTS.md

## Tooling & Commands

- **All dev tasks via `mise`** (see `mise.toml`). Never run raw cargo directly for standard workflows.
  - `mise install` — install tools (Rust 1.81.0, bun, Python, Go) + init submodules + bun install
  - `mise run build` — `cargo build --release`
  - `mise run test` — runs both `test:rust` + `test:js`
  - `mise run fmt` — `cargo fmt --all` + header application
  - `mise run check` / `mise run clippy` — `SKIP_WASM_BUILD=1` variants
  - `mise run init` — required first step after clone (submodules + deps)
- **Run node locally**: `cargo run --release -p setheum-node -- --dev --tmp --alice`
- **WASM builds** require nightly toolchain (init script sets nightly-2021-11-07); use `SKIP_WASM_BUILD=1` to skip for fast check/test.

## Feature Flags

Most operations need `--features with-ethereum-compatibility`:
- `mise run test:rust` already includes it: `SKIP_WASM_BUILD=1 cargo test --workspace --features with-ethereum-compatibility`
- Benchmarks: `--features runtime-benchmarks,with-ethereum-compatibility`
- Production WASM: `--features on-chain-release-build,no-metadata-docs`

## Workspace Structure

6 sub-repos under `repos/`:
| Repo | Language | Description |
|------|----------|-------------|
| `setheum/` | Rust | Core blockchain: **`setheum-node`** (binary at `node/src/main.rs`), **`setheum-runtime`** (runtime), 53+ custom pallets in `runtime-modules/` |
| `set-bft/` | Rust | Set-BFT consensus engine |
| `bridge/` | Rust + Go | Bridge (bridge-core, bridge-relayer) |
| `setheum-js/` | TS/JS | JS SDK |
| `sheyth/` | Rust | Sheyth smart contract framework (Ink! fork adapted for SheythVM) |
| `sheyth-vm/` | Rust | **SheythVM** — RISC-V based VM (PolkaVM fork), standalone workspace |
| `spinit/` | Rust | SheythVM contract testing toolkit (drink fork) |

Plus supporting crates:
- `sheyth-vm-uapi/` — Host function API types shared between Sheyth framework and SheythVM

Cargo workspace root `Cargo.toml` includes all members. Go workspace (`go.work`) covers bridge-core + bridge-relayer.

## Architecture — Smart Contracts

**SheythVM** (`repos/sheyth-vm/`) is a RISC-V based VM (forked from PolkaVM). Contracts are written in Rust, compiled to `.sheythvm` bytecode.

- **Pallet**: `pallet-sheyth-vm` at `runtime-modules/sheyth-vm/`
- **Precompiles**: 27 host functions in `pallet-sheyth-vm/src/precompiles.rs`:
  - *Solidity-compatible*: ecrecover, sha256, ripemd160, identity, modexp, bn128 add/mul/pairing
  - *Sheyth-native*: token (ERC20 interface), currency, DEX, oracle, NFT, schedule
- **Predeployed contracts**: at `predeploy-contracts/` — deployed at genesis (SEU, SEUSD, DEX)
- **Contract framework**: `repos/sheyth/` — adapted Ink! fork, now targets SheythVM via `sheyth-vm-uapi` instead of `pallet-contracts-uapi`
- **Testing**: `repos/spinit/` — Spinit (drink fork) uses `sheyth_vm::Engine` directly

## Architecture — Consensus

- **Aura** for block production
- **Set-BFT** (at `repos/set-bft/`) for finality
- **`finality-setbft/`** — Substrate node integration (wired in `node/src/service.rs`)
- **`module_setbft/`** — On-chain pallet (SessionManager, OneSessionHandler)

## Rust Conventions

- **rustfmt**: nightly channel required (CI uses nightly-2024-02-14). Config: hard_tabs, max_width 120, `Crate`-style imports, GPL3 header template.
- **rust-toolchain.toml**: channel = `1.88.0`. Components: `rustfmt`, `clippy`.
- **Substrate SDK pin**: `stable2506` branch for both polkadot-sdk and frontier.

## Testing

- Rust tests: `SKIP_WASM_BUILD=1 cargo test -p <crate>` (individual crate) or workspace-wide (can be slow).
- JS tests: `mise run test:js` (uses `bun test`).
- Bridge Rust: `cargo test -p bridge-relayer -p bridge-core`
- Bridge Go: `go test repos/bridge/...`
- Sheyth: `cargo test -p ink --all-features`
- E2E tests: standalone crate at `repos/setheum/e2e-tests/` (setheum-e2e-client).
- Runtime integration tests: `repos/setheum/runtime/tests/`.

## Architecture Notes

- **Runtime entry**: `repos/setheum/runtime/src/lib.rs` — `construct_runtime!` with all pallets.
- **Custom pallet dir**: `repos/setheum/runtime-modules/<pallet-name>/`.
- **Node entry**: `repos/setheum/node/src/main.rs` — handles CLI, chain spec, service.
- **Primitives**: `repos/setheum/primitives/` — shared types across pallets.
- **Release build**: Uses `srtool` + `subwasm` for reproducible WASM. Release branches named `release-<chain>-<version>`.

## Important Constraints

- **Submodules required**: `git submodule update --init --recursive` after clone.
- **Don't commit** to master; release branches follow `release-setheum-<version>` pattern.
- The `.agents/rules/setheum-builder.md` file contains general agent role instructions.
- License headers required on all Rust source files (GPL3 with Classpath exception). Use `mise run headers` to apply.
- **EVM/Frontier has been removed** — all smart contracts run on SheythVM (RISC-V).
- **SheythVM is a standalone workspace** (`repos/sheyth-vm/`) — not in root Cargo workspace.
