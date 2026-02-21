# ORML to Runtime Modules Merge Plan

This document outlines the plan to move and merge specific ORML modules from `orml/` into `runtime-modules/` safely and incrementally.

## Objectives
- Move requested ORML modules into `runtime-modules/`.
- Preserve existing functionality.
- Resolve conflicts with existing modules (`asset-registry`, `currencies`, etc.).
- Update project configuration to reflect the new structure.

## Modules Overview

### 1. Direct Move (No Name Conflict)
These modules do not currently exist in `runtime-modules/` (based on directory names) and can be moved directly.
- `auction` (Note: `ecdp-auctions` exists, but name differs)
- `benchmarking`
- `oracle` (Note: `edfis-oracle` exists, but name differs)
- `parameters`
- `rate-limit` (User noted `rate-limiter` exists, but `rate-limit` folder is safe to move)
- `rewards`
- `tokens`
- `traits`
- `utilities`

### 2. Conflicting Names (Requires Renaming/Resolution)
These modules already exist in `runtime-modules/` with the same directory name.
- `asset-registry`
- `currencies`
- `nft`
- `vesting`

## Execution Plan

### Phase 1: Preparation
1.  **Backup**: Ensure the current state is committed to git.
2.  **Verify Build**: Run `cargo check` to ensure the project is currently stable.

### Phase 2: Move Non-Conflicting Modules
For each module in the "Direct Move" list:
1.  Move the directory:
    ```bash
    mv orml/auction runtime-modules/
    mv orml/benchmarking runtime-modules/
    mv orml/oracle runtime-modules/
    mv orml/parameters runtime-modules/
    mv orml/rate-limit runtime-modules/
    mv orml/rewards runtime-modules/
    mv orml/tokens runtime-modules/
    mv orml/traits runtime-modules/
    mv orml/utilities runtime-modules/
    ```
2.  **Update `Cargo.toml` dependencies**:
    -   Scan the codebase for references to these modules (e.g., `path = "../../orml/tokens"`).
    -   Update paths to `path = "../../runtime-modules/tokens"`.
    -   Update the root workspace `Cargo.toml` `members` list if necessary (remove `orml/*`, add `runtime-modules/*` if not globbed).

### Phase 3: Move Conflicting Modules (Renaming Strategy)
To avoid overwriting existing custom modules, move the ORML versions into `runtime-modules` with an `orml-` prefix.

1.  Move and Rename:
    ```bash
    mv orml/asset-registry runtime-modules/orml-asset-registry
    mv orml/currencies runtime-modules/orml-currencies
    mv orml/nft runtime-modules/orml-nft
    mv orml/vesting runtime-modules/orml-vesting
    ```
2.  **Update Dependencies**:
    -   Any code specifically using the ORML version (referencing `orml/asset-registry`) should be updated to point to `runtime-modules/orml-asset-registry`.
    -   Existing code using `runtime-modules/asset-registry` remains unchanged for now.

### Phase 4: Integration and Cleanup
1.  **Workspace Update**:
    -   Check root `Cargo.toml`. Ensure `runtime-modules` glob includes the new folders.
    -   Remove `orml` references from `members` if empty or fully migrated.
2.  **Resolution of Duplicates** (Post-Merge Task):
    -   **Asset Registry**: Compare `runtime-modules/asset-registry` vs `runtime-modules/orml-asset-registry`. Decide to keep custom, replace with ORML, or merge features.
    -   **Currencies**: Compare `runtime-modules/currencies` vs `runtime-modules/orml-currencies`.
    -   **NFT**: Compare `runtime-modules/nft` vs `runtime-modules/orml-nft`.
    -   **Vesting**: Compare `runtime-modules/vesting` vs `runtime-modules/orml-vesting`.
    -   **Auction/Oracle**: Evaluate if `ecdp-auctions`/`edfis-oracle` should be replaced by or integrated with `auction`/`oracle`.

### Phase 5: Verification
1.  Run `cargo check` to catch path errors.
2.  Run tests for moved modules: `cargo test -p orml-tokens` (etc).

## Next Steps
- Execute the moves as described in Phase 2 and 3.
- Fix path dependencies in `Cargo.toml` files.