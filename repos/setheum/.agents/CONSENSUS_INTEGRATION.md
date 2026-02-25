# Consensus Integration: Aura + SetBFT

This document outlines the implementation plan and task progress for integrating Aura (for block authorship) and SetBFT (for finality) into the Setheum blockchain, replacing the legacy Babe/Grandpa consensus mechanism.

## Implementation Plan

### 1. Rebrand AlephBFT to SetBFT
- **Repository**: `repos/set-bft`
- **Actions**:
    - Rename directories and files: `aleph` -> `set`, `AlephBFT` -> `SetBFT`, `aleph-bft` -> `set-bft`.
    - Update text contents to reflect rebranding.
    - Status: [x] Done.

### 2. Implement Setheum Consensus (Aleph-Zero style)
- **Goal**: Adopt the consensus architecture used by Aleph-Zero (Aura + AlephBFT).
- **Runtime Changes**:
    - Replace `pallet-babe` and `pallet-grandpa` with `pallet-aura` and `module-setbft`.
    - Configure `Session` keys to include `AuraId`.
    - Integrate `module-committee-management`, `module-elections`, and `module-operations`.
- **Node Changes**:
    - Update `service.rs` to use `AuraBlockImport` and `AuraBlockAuthoring`.
    - Remove Grandpa finality gadget and replace with `finality-setbft`.
    - Update `rpc.rs` to remove Babe/Grandpa specific RPCs.
    - Update `command.rs` to remove light client support (incompatible with current SetBFT implementation).
    - Update `chain_spec.rs` to use `AuraConfig` and `AuraId`.
- **Status**: [x] Integrated, currently resolving dependency conflicts.

### 3. Hybrid Frontier EVM with Unified Accounts
- **Goal**: Implement Ethereum compatibility with Astar-style Unified Accounts and Acala-style Native Token support.
- **Approach**: Use Frontier pallets (`pallet-evm`, `pallet-ethereum`) with custom address mapping and precompiles for native currency support.
- **Status**: [ ] Pending completion of consensus integration.

---

## Task Progress

### Phase 1: Rebranding
- [x] Rename references in `repos/set-bft`.
- [x] Update Cargo manifests in `repos/set-bft`.
- [x] Verify basic compilation of the rebranded crates.

### Phase 2: Consensus Integration
- [x] Analyze Aleph-Zero runtime structure.
- [x] Replace Babe with Aura in `runtime/src/lib.rs`.
- [x] Replace Grandpa with SetBFT in `runtime/src/lib.rs`.
- [x] Update Session, Timestamp, and Authorship configs.
- [x] Refactor `node/src/service.rs` for Aura block import.
- [x] Refactor `node/src/rpc.rs` and `node/src/command.rs`.
- [x] Update `node/src/chain_spec.rs` using renaming script.
- [x] Update workspace and local `Cargo.toml` dependencies.
- [/] Update Polkadot SDK to `stable2506` and resolve dependency conflicts.
- [ ] Commit changes.

### Phase 3: Web3 Integrity (EVM)
- [ ] Analyze Astar Unified Accounts.
- [ ] Analyze Acala EVM Native Tokens.
- [ ] Integrate Frontier EVM.
- [ ] Implement Unified Address Mapping.
- [ ] Implement Native Currency Precompiles.
