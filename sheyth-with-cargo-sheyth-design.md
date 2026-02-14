# Technical Design: Merging cargo-sheyth into sheyth

This document outlines the architecture and steps for merging `cargo-sheyth` (a fork of `cargo-contract`) into `sheyth` (a fork of `ink!`).

## Objective
Merge the CLI tool (`cargo-sheyth`) and its supporting libraries into the core language repository (`sheyth`) to create a unified monorepo. This allows for better synchronization between the language (ink!) and the build tool (cargo-contract), and enables shared code reuse.

## Current Structure

### sheyth (ink! fork)
- Path: `/home/balqaasem/setheum/sheyth/`
- Version: 5.1.1
- Structure:
  - `crates/allocator`
  - `crates/env`
  - `crates/ink`
  - `crates/metadata` (ink_metadata)
  - ... and other ink! crates.

### cargo-sheyth (cargo-contract fork)
- Path: `/home/balqaasem/setheum/cargo-sheyth/`
- Version: 5.1.0 (depends on ink! 5.1.0)
- Structure:
  - `crates/cargo-contract`
  - `crates/build` (contract-build)
  - `crates/metadata` (contract-metadata)
  - `crates/transcode` (contract-transcode)
  - `crates/extrinsics` (contract-extrinsics)
  - `crates/analyze` (contract-analyze)

## Target Architecture

The merged repository will follow a unified workspace structure under `sheyth/`.

### Directory Layout
```
sheyth/
├── Cargo.toml (Workspace Root)
├── crates/
│   ├── ink/ (Core ink! crates)
│   ├── env/
│   ├── ...
│   ├── cargo-contract/ (Moved from cargo-sheyth)
│   ├── contract-build/ (Moved from cargo-sheyth/crates/build)
│   ├── contract-metadata/ (Moved from cargo-sheyth/crates/metadata)
│   ├── contract-transcode/
│   ├── contract-extrinsics/
│   └── contract-analyze/
└── ...
```

### Dependency Management
- **Shared Dependencies**: Move common dependencies (e.g., `serde`, `scale`, `anyhow`, `tracing`) to the workspace `[workspace.dependencies]` in `sheyth/Cargo.toml`.
- **Internal Linking**: `cargo-contract` and its libraries will point to the local `ink_*` crates (e.g., `ink_metadata`, `ink_env`) instead of fetching them from crates.io or git.
- **Version Alignment**: Align all internal versions to 5.1.1 (the version of `sheyth`).

## Reusable Elements
- Both repositories use `contract-metadata` and `ink_metadata` concepts. `contract-metadata` (from cargo-contract) is for the CLI/Tooling side, while `ink_metadata` (from ink!) is for the contract side. They are distinct but related.
- Shared utilities like `scale` codec versions and `subxt` versions will be unified in the workspace.

## Implementation Steps

### Phase 1: Preparation
1. [ ] Back up or ensure git state is clean.
2. [ ] Identify all crates in `cargo-sheyth/crates/`.

### Phase 2: Merging
1. [ ] Copy `cargo-sheyth/crates/*` to `sheyth/crates/`.
   - *Note: `metadata` from cargo-sheyth will be renamed to `contract-metadata` during copy to avoid conflict with `sheyth/crates/metadata` (ink_metadata).*
2. [ ] Update `sheyth/Cargo.toml` members to include the new crates.

### Phase 3: Integration & Refactoring
1. [ ] Update `Cargo.toml` of moved crates to point to local paths for internal dependencies.
2. [ ] Move common dependencies to workspace level.
3. [ ] Resolve version conflicts (especially `ink` dependencies in `cargo-contract`).

### Phase 4: Verification
1. [ ] Run `cargo check` on the entire workspace.
2. [ ] Run tests for both `ink!` crates and `cargo-contract` crates.
3. [ ] Verify `cargo-contract` can build a sample contract using the local `ink!` crates.

## Future Steps (After Confirmation)
1. [ ] Rename crates from `cargo-contract` to `cargo-sheyth` and `contract-*` to `sheyth-*`.
2. [ ] Update branding and documentation.
