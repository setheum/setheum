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
| `sheyth/` | Rust | Smart Contract framework (Ink! fork) |
| `spinit/` | Rust | Sheyth dev toolbox (drink) |

Cargo workspace root `Cargo.toml` includes all members. Go workspace (`go.work`) covers bridge-core + bridge-relayer.

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

- **Runtime entry**: `repos/setheum/runtime/src/lib.rs` — `construct_runtime!` with all 53+ pallets.
- **Custom pallet dir**: `repos/setheum/runtime-modules/<pallet-name>/`.
- **Node entry**: `repos/setheum/node/src/main.rs` — handles CLI, chain spec, service.
- **Primitives**: `repos/setheum/primitives/` — shared types used across pallets.
- **Consensus**: Aura block production + Grandpa finality + custom Set-BFT finality layer.
- **Release build**: Uses `srtool` + `subwasm` for reproducible WASM. Release branches named `release-<chain>-<version>`.

## Important Constraints

- **Submodules required**: `git submodule update --init --recursive` after clone.
- **Don't commit** to master; release branches follow `release-setheum-<version>` pattern.
- The `.agents/rules/setheum-builder.md` file contains general agent role instructions — prefer this file for repo-specific context.
- License headers required on all Rust source files (GPL3 with Classpath exception). Use `mise run headers` to apply.
