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
use wasm_opt::{
    Feature,
    OptimizationOptions,
    Pass,
};

use std::{
    fmt,
    path::PathBuf,
    str,
};

/// A helpful struct for interacting with Binaryen's `wasm-opt` tool.
pub struct WasmOptHandler {
    /// The optimization level that should be used when optimizing the Wasm binary.
    optimization_level: OptimizationPasses,
    /// Whether or not to keep debugging information in the final Wasm binary.
    keep_debug_symbols: bool,
}

impl WasmOptHandler {
    /// Generate a new instance of the handler.
    ///
    /// Fails if the `wasm-opt` binary is not installed on the system, or if an outdated
    /// `wasm-opt` binary is used (currently a version >= 99 is required).
    pub fn new(
        optimization_level: OptimizationPasses,
        keep_debug_symbols: bool,
    ) -> Result<Self> {
        Ok(Self {
            optimization_level,
            keep_debug_symbols,
        })
    }

    /// Attempts to perform optional Wasm optimization using Binaryen's `wasm-opt` tool.
    ///
    /// If successful, the optimized Wasm binary is written to `dest_wasm`.
    pub fn optimize(&self, original_wasm: &PathBuf, dest_wasm: &PathBuf) -> Result<()> {
        tracing::debug!(
            "Optimization level passed to wasm-opt: {}",
            self.optimization_level
        );

        OptimizationOptions::from(self.optimization_level)
            .mvp_features_only()
            // Since rustc 1.70 `SignExt` can't be disabled anymore. Hence we have to allow it,
            // in order that the Wasm binary containing these instructions can be loaded.
            .enable_feature(Feature::SignExt)
            // This pass will then remove any `signext` instructions in order that the resulting
            // Wasm binary is compatible with older versions of `pallet-contracts` which do not
            // support the `signext` instruction.
            .add_pass(Pass::SignextLowering)
            // the memory in our module is imported, `wasm-opt` needs to be told that
            // the memory is initialized to zeroes, otherwise it won't run the
            // memory-packing pre-pass.
            .zero_filled_memory(true)
            .debug_info(self.keep_debug_symbols)
            .run(original_wasm, dest_wasm)?;

        if !dest_wasm.exists() {
            return Err(anyhow::anyhow!(
                "Optimization failed, optimized wasm output file `{}` not found.",
                dest_wasm.display()
            ))
        }

        Ok(())
    }
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub enum OptimizationPasses {
    Zero,
    One,
    Two,
    Three,
    Four,
    S,
    #[default]
    Z,
}

impl fmt::Display for OptimizationPasses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            OptimizationPasses::Zero => "0",
            OptimizationPasses::One => "1",
            OptimizationPasses::Two => "2",
            OptimizationPasses::Three => "3",
            OptimizationPasses::Four => "4",
            OptimizationPasses::S => "s",
            OptimizationPasses::Z => "z",
        };
        write!(f, "{out}")
    }
}

impl str::FromStr for OptimizationPasses {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        // We need to replace " here, since the input string could come
        // from either the CLI or the `Cargo.toml` profile section.
        // If it is from the profile it could e.g. be "3" or 3.
        let normalized_input = input.replace('"', "").to_lowercase();
        match normalized_input.as_str() {
            "0" => Ok(OptimizationPasses::Zero),
            "1" => Ok(OptimizationPasses::One),
            "2" => Ok(OptimizationPasses::Two),
            "3" => Ok(OptimizationPasses::Three),
            "4" => Ok(OptimizationPasses::Four),
            "s" => Ok(OptimizationPasses::S),
            "z" => Ok(OptimizationPasses::Z),
            _ => anyhow::bail!("Unknown optimization passes for option {}", input),
        }
    }
}

impl From<String> for OptimizationPasses {
    fn from(str: String) -> Self {
        <OptimizationPasses as str::FromStr>::from_str(&str).expect("conversion failed")
    }
}

impl From<OptimizationPasses> for OptimizationOptions {
    fn from(passes: OptimizationPasses) -> OptimizationOptions {
        match passes {
            OptimizationPasses::Zero => OptimizationOptions::new_opt_level_0(),
            OptimizationPasses::One => OptimizationOptions::new_opt_level_1(),
            OptimizationPasses::Two => OptimizationOptions::new_opt_level_2(),
            OptimizationPasses::Three => OptimizationOptions::new_opt_level_3(),
            OptimizationPasses::Four => OptimizationOptions::new_opt_level_4(),
            OptimizationPasses::S => OptimizationOptions::new_optimize_for_size(),
            OptimizationPasses::Z => {
                OptimizationOptions::new_optimize_for_size_aggressively()
            }
        }
    }
}

/// Result of the optimization process.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptimizationResult {
    /// The original Wasm size.
    pub original_size: f64,
    /// The Wasm size after optimizations have been applied.
    pub optimized_size: f64,
}
