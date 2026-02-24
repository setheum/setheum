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

//! This module provides simple utilities for loading and parsing `.contract` files in context of `drink` tests.

use std::{path::PathBuf, sync::Arc};

use contract_metadata::ContractMetadata;
use contract_transcode::ContractMessageTranscoder;

use crate::{DrinkResult, Error};

/// A struct representing the result of parsing a `.contract` bundle file.
///
/// It can be used with the following methods of the `Session` struct:
/// - `deploy_bundle`
/// - `deploy_bundle_and`
/// - `upload_bundle`
/// - `upload_bundle_and`
#[derive(Clone)]
pub struct ContractBundle {
    /// Binary of the contract.
    pub binary: Vec<u8>,
    /// Transcoder derived from the ABI/metadata
    pub transcoder: Arc<ContractMessageTranscoder>,
}

impl ContractBundle {
    /// Load and parse the information in a `.contract` bundle under `path`, producing a
    /// `ContractBundle` struct.
    pub fn load<P>(path: P) -> DrinkResult<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let metadata: ContractMetadata = ContractMetadata::load(&path).map_err(|e| {
            Error::BundleLoadFailed(format!("Failed to load the contract file:\n{e:?}"))
        })?;

        let ink_metadata = serde_json::from_value(serde_json::Value::Object(metadata.abi))
            .map_err(|e| {
                Error::BundleLoadFailed(format!(
                    "Failed to parse metadata from the contract file:\n{e:?}"
                ))
            })?;

        let transcoder = Arc::new(ContractMessageTranscoder::new(ink_metadata));

        let binary = metadata
            .source
            .contract_binary
            .ok_or(Error::BundleLoadFailed(
                "Failed to get the WASM blob from the contract file".to_string(),
            ))?
            .0;

        Ok(Self { binary, transcoder })
    }

    /// Load the `.contract` bundle (`contract_file_name`) located in the `project_dir`` working directory.
    ///
    /// This is meant to be used predominantly by the `local_contract_file!` macro.
    pub fn local(project_dir: &str, contract_file_name: String) -> Self {
        let mut path = PathBuf::from(project_dir);
        path.push("target");
        path.push("ink");
        path.push(contract_file_name);
        Self::load(path).expect("Loading the local bundle failed")
    }
}

/// A convenience macro that allows you to load a bundle found in the target directory
/// of the current project.
#[macro_export]
macro_rules! local_contract_file {
    () => {
        drink::session::ContractBundle::local(
            env!("CARGO_MANIFEST_DIR"),
            env!("CARGO_CRATE_NAME").to_owned() + ".contract",
        )
    };
}
