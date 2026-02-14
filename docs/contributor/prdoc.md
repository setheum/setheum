# PRDocs

PRDocs are small YAML files that describe user-visible changes at the crate level.
They are used to automate the generation of `CHANGELOG.md` and track version bumps.
This system is inspired by the Polkadot-SDK PRDoc process.

## When to add a PRDoc

Add a PRDoc whenever a change impacts users of any crate in the workspace.
This includes:
-   New features or APIs.
-   Breaking changes.
-   Bug fixes.
-   Deprecations.
-   Security improvements.

## File location

Create a `.prdoc` file under the `prdoc/` directory. You can organize them by version subdirectories if desired.
Example: `prdoc/v1.0.0/fix-ink-env-metadata.prdoc`

## Format

PRDocs use YAML format.

```yaml
title: "Add XCM send support to ink_env"
doc: |
  Detailed explanation of the change. This can span multiple lines
  and provides more context than the short title.
audience:
  - Contract Developer
  - Runtime Developer

crates:
  - name: ink_env
    section: Added
    note: Add xcm_send for contracts to submit XCM messages
    bump: minor
```

### Fields

-   `title`: (Required) A short summary of the change.
-   `doc`: (Required) A more detailed description of the change.
-   `audience`: (Optional) A list of impacted groups (e.g., Contract Developer, Runtime Developer, Node Operator).
-   `crates`: (Required) A list of impacted crates.
    -   `name`: (Required) The name of the crate (must match a workspace crate).
    -   `section`: (Required) The category of the change. One of:
        -   `Breaking`: Breaking changes.
        -   `Added`: New features.
        -   `Changed`: Changes in existing functionality.
        -   `Fixed`: Bug fixes.
        -   `Removed`: Removed features.
        -   `Deprecated`: Deprecated features.
        -   `Security`: Security improvements.
    -   `note`: (Required) The description that will appear in the CHANGELOG.
    -   `bump`: (Required) The semver bump type: `major`, `minor`, `patch`, or `none`.

## Automation

The `scripts/prdoc.py` tool provides commands to manage PRDocs. You can use them directly or via `mise`.

### Scaffolding

To efficiently generate PRDocs from your git history (since the last version tag):

```bash
# Via mise (Recommended)
mise run prdoc:scaffold

# Or directly
python3 scripts/prdoc.py scaffold
```

This will:
1.  Identify all commits since the last tag.
2.  Map changed files to specific crates in the workspace.
3.  Create draft `.prdoc` files in `prdoc/drafts/` with pre-filled titles and affected crates.

You can also specify a specific revision:
```bash
mise run prdoc:scaffold -- --since HEAD~5
```

### Validation

To validate all PRDocs in the repository:

```bash
# Via mise (Recommended)
mise run prdoc:validate

# Or directly
python3 scripts/prdoc.py validate
```

### Generation

To generate the `CHANGELOG.md` entries from PRDocs:

```bash
# Via mise (Recommended)
mise run prdoc:generate

# Or directly
python3 scripts/prdoc.py generate
```

This will update the `[Unreleased]` section of the root `CHANGELOG.md` with entries grouped by crate and section.

## CI/CD Automation

The PRDoc system is integrated with GitHub Actions to ensure consistency and automate updates.

### 1. PR Enforcement (`prdoc-check.yml`)
Every Pull Request that modifies code or configuration must include a `.prdoc` file in the `prdoc/` directory.
-   If a PR touches code but lacks a PRDoc, the check will fail.
-   For PRs that don't need a changelog entry (e.g., pure documentation changes), add the `no-changelog` label to the PR to bypass the check.

### 2. Automated Publishing (`prdoc-publish.yml`)
When a PR is merged into `main` or `master`:
-   The workflow automatically runs `mise run prdoc:generate`.
-   The updated `CHANGELOG.md` is committed and pushed back to the branch.
-   This ensures the `[Unreleased]` section is always up-to-date with the latest merged changes.
