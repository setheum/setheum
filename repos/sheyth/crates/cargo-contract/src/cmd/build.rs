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

use anyhow::Result;
use contract_build::{
    BuildArtifacts,
    BuildMode,
    BuildResult,
    ExecuteArgs,
    Features,
    ImageVariant,
    ManifestPath,
    Network,
    OptimizationPasses,
    OutputType,
    Target,
    UnstableFlags,
    UnstableOptions,
    Verbosity,
    VerbosityFlags,
};
use std::{
    convert::TryFrom,
    path::PathBuf,
};

/// Executes build of the smart contract which produces a Wasm binary that is ready for
/// deploying.
///
/// It does so by invoking `cargo build` and then post processing the final binary.
#[derive(Debug, clap::Args)]
#[clap(name = "build")]
pub struct BuildCommand {
    /// Path to the `Cargo.toml` of the contract to build
    #[clap(long, value_parser)]
    manifest_path: Option<PathBuf>,
    /// By default the contract is compiled with debug functionality
    /// included. This enables the contract to output debug messages,
    /// but increases the contract size and the amount of gas used.
    ///
    /// A production contract should always be build in `release` mode!
    /// Then no debug functionality is compiled into the contract.
    #[clap(long = "release")]
    build_release: bool,
    /// Build offline
    #[clap(long = "offline")]
    build_offline: bool,
    /// Performs extra linting checks for ink! specific issues during the build process.
    ///
    /// Basic clippy lints are deemed important and run anyways.
    #[clap(long)]
    lint: bool,
    /// Which build artifacts to generate.
    ///
    /// - `all`: Generate the Wasm, the metadata and a bundled `<name>.contract` file.
    ///
    /// - `code-only`: Only the Wasm is created, generation of metadata and a bundled
    ///   `<name>.contract` file is skipped.
    ///
    /// - `check-only`: No artifacts produced: runs the `cargo check` command for the
    ///   Wasm target, only checks for compilation errors.
    #[clap(long = "generate", value_enum, default_value = "all")]
    build_artifact: BuildArtifacts,
    #[clap(flatten)]
    features: Features,
    #[clap(flatten)]
    verbosity: VerbosityFlags,
    #[clap(flatten)]
    unstable_options: UnstableOptions,
    /// Number of optimization passes, passed as an argument to `wasm-opt`.
    ///
    /// - `0`: execute no optimization passes
    ///
    /// - `1`: execute 1 optimization pass (quick & useful opts, useful for iteration
    ///   builds)
    ///
    /// - `2`, execute 2 optimization passes (most opts, generally gets most perf)
    ///
    /// - `3`, execute 3 optimization passes (spends potentially a lot of time
    ///   optimizing)
    ///
    /// - `4`, execute 4 optimization passes (also flatten the IR, which can take a lot
    ///   more time and memory but is useful on more nested / complex / less-optimized
    ///   input)
    ///
    /// - `s`, execute default optimization passes, focusing on code size
    ///
    /// - `z`, execute default optimization passes, super-focusing on code size
    ///
    /// - The default value is `z`
    ///
    /// - It is possible to define the number of optimization passes in the
    ///   `[package.metadata.contract]` of your `Cargo.toml` as e.g. `optimization-passes
    ///   = "3"`. The CLI argument always takes precedence over the profile value.
    #[clap(long)]
    optimization_passes: Option<OptimizationPasses>,
    /// Do not remove symbols (Wasm name section) when optimizing.
    ///
    /// This is useful if one wants to analyze or debug the optimized binary.
    #[clap(long)]
    keep_debug_symbols: bool,
    /// Export the build output in JSON format.
    #[clap(long, conflicts_with = "verbose")]
    output_json: bool,
    /// Don't perform wasm validation checks e.g. for permitted imports.
    #[clap(long)]
    skip_wasm_validation: bool,
    /// Which bytecode to build the contract into.
    #[clap(long, default_value = "wasm")]
    target: Target,
    /// The maximum number of pages available for a wasm contract to allocate.
    #[clap(long, default_value_t = contract_build::DEFAULT_MAX_MEMORY_PAGES)]
    max_memory_pages: u64,
    /// Executes the build inside a docker container to produce a verifiable bundle.
    /// Requires docker daemon running.
    #[clap(long, default_value_t = false)]
    verifiable: bool,
    /// Specify a custom image for the verifiable build
    #[clap(long, default_value = None)]
    image: Option<String>,
}

impl BuildCommand {
    pub fn exec(&self) -> Result<BuildResult> {
        let manifest_path = ManifestPath::try_from(self.manifest_path.as_ref())?;
        let unstable_flags: UnstableFlags =
            TryFrom::<&UnstableOptions>::try_from(&self.unstable_options)?;
        let verbosity = TryFrom::<&VerbosityFlags>::try_from(&self.verbosity)?;

        let build_mode = if self.verifiable {
            BuildMode::Verifiable
        } else {
            match self.build_release {
                true => BuildMode::Release,
                false => BuildMode::Debug,
            }
        };

        let network = match self.build_offline {
            true => Network::Offline,
            false => Network::Online,
        };

        let output_type = match self.output_json {
            true => OutputType::Json,
            false => OutputType::HumanReadable,
        };

        if self.image.is_some() && build_mode != BuildMode::Verifiable {
            anyhow::bail!("--image flag can only be used with verifiable builds!");
        }

        let image = match &self.image {
            Some(i) => ImageVariant::Custom(i.clone()),
            None => ImageVariant::Default,
        };

        let args = ExecuteArgs {
            manifest_path,
            verbosity,
            build_mode,
            features: self.features.clone(),
            network,
            build_artifact: self.build_artifact,
            unstable_flags,
            optimization_passes: self.optimization_passes,
            keep_debug_symbols: self.keep_debug_symbols,
            extra_lints: self.lint,
            output_type,
            skip_wasm_validation: self.skip_wasm_validation,
            target: self.target,
            max_memory_pages: self.max_memory_pages,
            image,
        };
        contract_build::execute(args)
    }
}

#[derive(Debug, clap::Args)]
#[clap(name = "check")]
pub struct CheckCommand {
    /// Path to the `Cargo.toml` of the contract to build
    #[clap(long, value_parser)]
    manifest_path: Option<PathBuf>,
    #[clap(flatten)]
    verbosity: VerbosityFlags,
}

impl CheckCommand {
    pub fn exec(&self) -> Result<BuildResult> {
        let manifest_path = ManifestPath::try_from(self.manifest_path.as_ref())?;
        let verbosity: Verbosity = TryFrom::<&VerbosityFlags>::try_from(&self.verbosity)?;

        let args = ExecuteArgs {
            manifest_path,
            verbosity,
            build_mode: BuildMode::Debug,
            features: Default::default(),
            network: Network::default(),
            build_artifact: BuildArtifacts::CheckOnly,
            unstable_flags: Default::default(),
            optimization_passes: Some(OptimizationPasses::Zero),
            keep_debug_symbols: false,
            extra_lints: false,
            output_type: OutputType::default(),
            skip_wasm_validation: false,
            target: Default::default(),
            max_memory_pages: 0,
            image: ImageVariant::Default,
        };

        contract_build::execute(args)
    }
}
