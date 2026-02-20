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
use std::{
    fs,
    path::Path,
};
use toml::{
    Table,
    Value,
};

/// Generates a cargo workspace package `metadata-gen` which will be invoked via `cargo
/// run` to generate contract metadata.
///
/// # Note
///
/// `ink!` dependencies are copied from the containing contract workspace to ensure the
/// same versions are utilized.
pub(super) fn generate_package<P: AsRef<Path>>(
    target_dir: P,
    contract_package_name: &str,
    mut ink_crate_dependency: Table,
    contract_features: &Table,
) -> Result<()> {
    let dir = target_dir.as_ref();
    tracing::debug!(
        "Generating metadata package for {} in {}",
        contract_package_name,
        dir.display()
    );

    let cargo_toml = include_str!("../../templates/generate-metadata/_Cargo.toml");
    let main_rs = include_str!("../../templates/generate-metadata/main.rs");

    let mut cargo_toml: Table = toml::from_str(cargo_toml)?;
    let deps = cargo_toml
        .get_mut("dependencies")
        .expect("[dependencies] section specified in the template")
        .as_table_mut()
        .expect("[dependencies] is a table specified in the template");

    // initialize contract dependency
    let contract = deps
        .get_mut("contract")
        .expect("contract dependency specified in the template")
        .as_table_mut()
        .expect("contract dependency is a table specified in the template");
    contract.insert("package".into(), contract_package_name.into());

    // make ink_metadata dependency use default features
    ink_crate_dependency.remove("default-features");
    ink_crate_dependency.remove("features");
    ink_crate_dependency.remove("optional");

    // add ink dependencies copied from contract manifest
    deps.insert("ink".into(), ink_crate_dependency.into());

    // add features from contract
    let features = cargo_toml
        .entry("features")
        .or_insert(Value::Table(Default::default()))
        .as_table_mut()
        .ok_or_else(|| anyhow::anyhow!("features should be a table"))?;

    for (feature, _) in contract_features {
        if feature != "default" && feature != "std" {
            features.insert(
                feature.to_string(),
                Value::Array(vec![format!("contract/{feature}").into()]),
            );
        }
    }

    let cargo_toml = toml::to_string(&cargo_toml)?;
    fs::write(dir.join("Cargo.toml"), cargo_toml)?;
    fs::write(dir.join("main.rs"), main_rs)?;
    Ok(())
}
