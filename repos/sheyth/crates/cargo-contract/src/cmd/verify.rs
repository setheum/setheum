// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use anyhow::{
    Context,
    Result,
};
use colored::Colorize;
use contract_build::{
    code_hash,
    execute,
    util::decode_hex,
    verbose_eprintln,
    BuildArtifacts,
    BuildInfo,
    BuildMode,
    ExecuteArgs,
    ImageVariant,
    ManifestPath,
    Verbosity,
    VerbosityFlags,
};
use contract_metadata::{
    CodeHash,
    ContractMetadata,
};

use regex::Regex;
use std::{
    fs::File,
    path::PathBuf,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Checks if a contract in the given workspace matches that of a reference contract.
#[derive(Debug, clap::Args)]
#[clap(name = "verify")]
pub struct VerifyCommand {
    /// Path to the `Cargo.toml` of the contract to verify.
    #[clap(long, value_parser)]
    manifest_path: Option<PathBuf>,
    /// The reference Wasm contract (`*.contract`) that the workspace will be checked
    /// against.
    #[clap(long)]
    contract: Option<PathBuf>,
    /// The reference Wasm contract binary (`*.wasm`) that the workspace will be checked
    /// against.
    #[clap(long, conflicts_with = "contract")]
    wasm: Option<PathBuf>,
    /// Denotes if output should be printed to stdout.
    #[clap(flatten)]
    verbosity: VerbosityFlags,
    /// Output the result in JSON format
    #[clap(long, conflicts_with = "verbose")]
    output_json: bool,
}

impl VerifyCommand {
    pub fn run(&self) -> Result<VerificationResult> {
        let manifest_path = ManifestPath::try_from(self.manifest_path.as_ref())?;
        let verbosity: Verbosity = TryFrom::<&VerbosityFlags>::try_from(&self.verbosity)?;
        if let Some(path) = &self.contract {
            self.verify_contract(manifest_path, verbosity, path)
        } else if let Some(path) = &self.wasm {
            self.verify_wasm(manifest_path, verbosity, path)
        } else {
            anyhow::bail!("Either --wasm or --contract must be specified")
        }
    }

    /// Verify `.wasm` binary.
    fn verify_wasm(
        &self,
        manifest_path: ManifestPath,
        verbosity: Verbosity,
        path: &PathBuf,
    ) -> Result<VerificationResult> {
        // 1. Read code hash binary from the path.
        let ref_buffer = std::fs::read(path)
            .context(format!("Failed to read contract binary {}", path.display()))?;

        let reference_code_hash = CodeHash(code_hash(&ref_buffer));

        // 2. Call `cargo contract build` in the release mode.
        let args = ExecuteArgs {
            manifest_path: manifest_path.clone(),
            verbosity,
            optimization_passes: Some(contract_build::OptimizationPasses::Z),
            build_mode: BuildMode::Release,
            build_artifact: BuildArtifacts::CodeOnly,
            extra_lints: false,
            ..Default::default()
        };

        let build_result = execute(args)?;

        // 4. Grab the code hash from the built contract and compare it with the reference
        //    one.
        let built_wasm_path = if let Some(m) = build_result.dest_wasm {
            m
        } else {
            // Since we're building the contract ourselves this should always be
            // populated, but we'll bail out here just in case.
            anyhow::bail!("\nThe workspace contract does not contain a Wasm binary,\n\
                therefore we are unable to verify the contract."
                .to_string()
                .bright_yellow())
        };

        let target_buffer = std::fs::read(&built_wasm_path).context(format!(
            "Failed to read contract binary {}",
            built_wasm_path.display()
        ))?;

        let output_code_hash = CodeHash(code_hash(&target_buffer));

        if output_code_hash != reference_code_hash {
            anyhow::bail!(format!(
                "\nFailed to verify the authenticity of wasm binary at {} against the workspace \n\
                found at {}.\n Expected {}, found {}",
                format!("`{}`", path.display()).bright_white(),
                format!("`{}`", built_wasm_path.display()).bright_white(),
                format!("{}", reference_code_hash).bright_white(),
                format!("{}", output_code_hash).bright_white())
            );
        }

        Ok(VerificationResult {
            is_verified: true,
            image: None,
            contract: built_wasm_path.display().to_string(),
            reference_contract: path.display().to_string(),
            output_json: self.output_json,
            verbosity,
        })
    }

    /// Verify the `.contract` bundle.
    fn verify_contract(
        &self,
        manifest_path: ManifestPath,
        verbosity: Verbosity,
        path: &PathBuf,
    ) -> Result<VerificationResult> {
        // 1. Read the given metadata, and pull out the `BuildInfo`
        let file = File::open(path)
            .context(format!("Failed to open contract bundle {}", path.display()))?;

        let metadata: ContractMetadata = serde_json::from_reader(&file).context(
            format!("Failed to deserialize contract bundle {}", path.display()),
        )?;
        let build_info = if let Some(info) = metadata.source.build_info {
            info
        } else {
            anyhow::bail!(
                "\nThe metadata does not contain any build information which can be used to \
                verify a contract."
                .to_string()
                .bright_yellow()
            )
        };

        let build_info: BuildInfo =
            serde_json::from_value(build_info.into()).context(format!(
                "Failed to deserialize the build info from {}",
                path.display()
            ))?;

        tracing::debug!(
            "Parsed the following build info from the metadata: {:?}",
            &build_info,
        );

        let build_mode = if metadata.image.is_some() {
            BuildMode::Verifiable
        } else {
            build_info.build_mode
        };

        // 2. Check that the build info from the metadata matches our current setup.
        // if the build mode is `Verifiable` we skip
        if build_mode != BuildMode::Verifiable {
            let expected_rust_toolchain = build_info.rust_toolchain;
            let rust_toolchain = contract_build::util::rust_toolchain()
                .expect("`rustc` always has a version associated with it.");

            validate_toolchain_name(&expected_rust_toolchain)?;
            validate_toolchain_name(&rust_toolchain)?;

            let rustc_matches = rust_toolchain == expected_rust_toolchain;
            let mismatched_rustc = format!(
                "\nYou are trying to `verify` a contract using the following toolchain:\n\
                {rust_toolchain}\n\n\
                However, the original contract was built using this one:\n\
                {expected_rust_toolchain}\n\n\
                Please install the correct toolchain and re-run the `verify` command:\n\
                rustup install {expected_rust_toolchain}");
            anyhow::ensure!(rustc_matches, mismatched_rustc.bright_yellow());

            let expected_cargo_contract_version = build_info.cargo_contract_version;
            let cargo_contract_version = semver::Version::parse(VERSION)?;

            // Note, assuming both versions of `cargo-contract` were installed with the
            // same lockfile (e.g `--locked`) then the versions of `wasm-opt`
            // should also match.
            let cargo_contract_matches =
                cargo_contract_version == expected_cargo_contract_version;
            let mismatched_cargo_contract = format!(
                "\nYou are trying to `verify` a contract using `cargo-contract` version \
                `{cargo_contract_version}`.\n\n\
                However, the original contract was built using `cargo-contract` version \
                `{expected_cargo_contract_version}`.\n\n\
                Please install the matching version and re-run the `verify` command.\n\
                cargo install --force --locked cargo-contract --version {expected_cargo_contract_version}",
            );
            anyhow::ensure!(
                cargo_contract_matches,
                mismatched_cargo_contract.bright_yellow()
            );
        }

        // 3a. Call `cargo contract build` with the `BuildInfo` from the metadata.
        let args = ExecuteArgs {
            manifest_path: manifest_path.clone(),
            verbosity,
            build_mode,
            build_artifact: BuildArtifacts::All,
            optimization_passes: Some(build_info.wasm_opt_settings.optimization_passes),
            keep_debug_symbols: build_info.wasm_opt_settings.keep_debug_symbols,
            image: ImageVariant::from(metadata.image.clone()),
            extra_lints: false,
            ..Default::default()
        };

        let build_result = execute(args)?;

        // 4. Grab the code hash from the built contract and compare it with the reference
        //    code hash.
        //
        //    We compute the hash of the reference code here, instead of relying on
        //    the `source.hash` field in the metadata. This is because the `source.hash`
        //    field could have been manipulated; we want to be sure that _the code_ of
        //    both contracts is equal.
        let reference_wasm_blob = decode_hex(
            &metadata
                .source
                .wasm
                .expect("no source.wasm field exists in metadata")
                .to_string(),
        )
        .expect("decoding the source.wasm hex failed");
        let reference_code_hash = CodeHash(code_hash(&reference_wasm_blob));
        let built_contract_path = if let Some(m) = build_result.metadata_result {
            m
        } else {
            // Since we're building the contract ourselves this should always be
            // populated, but we'll bail out here just in case.
            anyhow::bail!(
                "\nThe metadata for the workspace contract does not contain a Wasm binary,\n\
                therefore we are unable to verify the contract."
                .to_string()
                .bright_yellow()
            )
        };

        let target_bundle = &built_contract_path.dest_bundle;

        let file = File::open(target_bundle.clone()).context(format!(
            "Failed to open contract bundle {}",
            target_bundle.display()
        ))?;
        let built_contract: ContractMetadata =
            serde_json::from_reader(file).context(format!(
                "Failed to deserialize contract bundle {}",
                target_bundle.display()
            ))?;

        let target_code_hash = built_contract.source.hash;

        if reference_code_hash != target_code_hash {
            verbose_eprintln!(
                verbosity,
                "Expected code hash in reference contract ({}): {}\nGot Code Hash: {}\n",
                &path.display(),
                &reference_code_hash,
                &target_code_hash
            );
            anyhow::bail!(format!(
                "\nFailed to verify `{}` against the workspace at `{}`: the hashed Wasm blobs are not matching.",
                format!("{}", &path.display()).bright_white(),
                format!("{}", manifest_path.as_ref().display()).bright_white()
            )
            .bright_red());
        }

        // check that the metadata hash is the same as reference_code_hash
        if reference_code_hash != metadata.source.hash {
            verbose_eprintln!(
                verbosity,
                "Expected code hash in reference metadata ({}): {}\nGot Code Hash: {}\n",
                &path.display(),
                &reference_code_hash,
                &metadata.source.hash
            );
            anyhow::bail!(format!(
                "\nThe reference contract `{}` metadata is corrupt: the source.hash does not match the source.wasm hash.",
                format!("{}", &path.display()).bright_white()
            )
            .bright_red());
        }

        Ok(VerificationResult {
            is_verified: true,
            image: metadata.image,
            contract: target_bundle.display().to_string(),
            reference_contract: path.display().to_string(),
            output_json: self.output_json,
            verbosity,
        })
    }
}

/// The result of verification process
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VerificationResult {
    pub is_verified: bool,
    pub image: Option<String>,
    pub contract: String,
    pub reference_contract: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub output_json: bool,
    #[serde(skip_serializing, skip_deserializing)]
    pub verbosity: Verbosity,
}

impl VerificationResult {
    /// Display the result in a fancy format
    pub fn display(&self) -> String {
        format!(
            "\n{} {} against reference contract {}",
            "Successfully verified contract".bright_green().bold(),
            format!("`{}`", &self.contract).bold(),
            format!("`{}`!", &self.reference_contract).bold()
        )
    }

    /// Display the build results in a pretty formatted JSON string.
    pub fn serialize_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

/// Validates that the passed `toolchain` is a valid Rust toolchain.
///
/// # Developers Note
///
/// Strictly speaking Rust has not yet defined rules for legal toolchain
/// names. See https://github.com/rust-lang/rustup/issues/4059 for more
/// details.
///
/// We took a "good enough" approach and restrict valid toolchain names
/// to established ones.
fn validate_toolchain_name(toolchain: &str) -> Result<()> {
    let re = Regex::new(r"^[a-zA-Z._\-0-9]+$").expect("failed creating regex");
    if re.is_match(toolchain) {
        return Ok(());
    }
    anyhow::bail!("Invalid toolchain name: {}", toolchain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_toolchain_names() {
        assert!(validate_toolchain_name("nightly").is_ok());
        assert!(validate_toolchain_name("stable").is_ok());
        assert!(validate_toolchain_name("beta").is_ok());

        assert!(validate_toolchain_name("nightly-2023-01-01").is_ok());
        assert!(validate_toolchain_name("beta-2024-01-02").is_ok());
        assert!(validate_toolchain_name("stable-2022-03-03").is_ok());

        assert!(validate_toolchain_name("1.56.0").is_ok());
        assert!(validate_toolchain_name("1.70").is_ok());

        assert!(validate_toolchain_name("1.70-aarch64-apple-darwin").is_ok());
        assert!(
            validate_toolchain_name("nightly-2024-11-05-aarch64-apple-darwin").is_ok()
        );
        assert!(validate_toolchain_name("stable-x86_64-unknown-linux-gnu").is_ok());
    }

    #[test]
    fn invalid_toolchain_names() {
        assert!(validate_toolchain_name("https://sh.rust-toolchain.rs").is_err());
        assert!(validate_toolchain_name("_ $").is_err());
        assert!(validate_toolchain_name(
            "nightly', please install https://sh.rust-toolchain.rs"
        )
        .is_err());
    }
}
