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

use std::{
    fs::File,
    path::PathBuf,
};

use anyhow::{
    anyhow,
    Context,
    Result,
};
use colored::Colorize;
use contract_build::{
    Verbosity,
    VerbosityFlags,
};
use schemars::schema_for;

#[derive(Debug, Clone, Default, clap::ValueEnum)]
#[clap(name = "metadata")]
enum Metadata {
    /// Represents the outer schema format of the contract
    #[clap(name = "outer")]
    #[default]
    Outer,
    /// Represents the inner schema format of the contract.
    /// Contains specification of the ink! contract.
    #[clap(name = "inner")]
    Inner,
}

/// Checks if a contract in the given workspace matches that of a reference contract.
#[derive(Debug, clap::Args)]
pub struct GenerateSchemaCommand {
    /// What type of metadata to generate.
    #[clap(long, value_enum, default_value = "outer")]
    metadata: Metadata,
}

impl GenerateSchemaCommand {
    pub fn run(&self) -> Result<String> {
        let schema = match self.metadata {
            Metadata::Outer => schema_for!(ink_metadata::InkProject),
            Metadata::Inner => schema_for!(ink_metadata::ConstructorSpec),
        };
        let pretty_string = serde_json::to_string_pretty(&schema)?;

        Ok(pretty_string)
    }
}

/// Verifies the metadata of the given contract against the schema file.
#[derive(Debug, clap::Args)]
pub struct VerifySchemaCommand {
    /// The path to metadata
    #[clap(long, value_parser)]
    schema: PathBuf,
    /// The .contract path to verify the metadata
    #[clap(name = "bundle", long, value_parser)]
    contract_bundle: Option<PathBuf>,
    /// What type of metadata to verify.
    #[clap(long, conflicts_with = "bundle", value_parser)]
    metadata: Option<PathBuf>,
    /// Denotes if output should be printed to stdout.
    #[clap(flatten)]
    verbosity: VerbosityFlags,
    /// Output the result in JSON format
    #[clap(long, conflicts_with = "verbose")]
    output_json: bool,
}

impl VerifySchemaCommand {
    pub fn run(&self) -> Result<SchemaVerificationResult> {
        let verbosity: Verbosity = TryFrom::<&VerbosityFlags>::try_from(&self.verbosity)?;

        let mut metadata = serde_json::Value::Null;
        let mut metadata_source = String::new();

        // 1a. Extract given metadata from .contract bundle
        if let Some(path) = &self.contract_bundle {
            let file = File::open(path)
                .context(format!("Failed to open contract bundle {}", path.display()))?;

            let mut contract_metadata: contract_metadata::ContractMetadata =
                serde_json::from_reader(&file).context(format!(
                    "Failed to deserialize contract bundle {}",
                    path.display()
                ))?;
            contract_metadata.remove_source_wasm_attribute();

            metadata = serde_json::value::to_value(contract_metadata)?;
            metadata_source = path.display().to_string();
        }

        // 1b. Read metadata file
        if let Some(path) = &self.metadata {
            let file = File::open(path)
                .context(format!("Failed to open metadata file {}", path.display()))?;

            let contract_metadata: contract_metadata::ContractMetadata =
                serde_json::from_reader(&file).context(format!(
                    "Failed to deserialize metadata file {}",
                    path.display()
                ))?;

            metadata = serde_json::value::to_value(contract_metadata)?;
            metadata_source = path.display().to_string();
        }

        // 2. Open schema file
        let path = &self.schema;
        let file = File::open(path)
            .context(format!("Failed to open schema file {}", path.display()))?;

        let schema: serde_json::Value = serde_json::from_reader(&file).context(
            format!("Failed to deserialize schema file {}", path.display()),
        )?;

        // 3. Validate and display error if any
        jsonschema::validate(&schema, &metadata).map_err(|err| {
            anyhow!(format!("Error during schema validation: {}\n", err))
        })?;

        Ok(SchemaVerificationResult {
            is_verified: true,
            metadata_source,
            schema: self.schema.display().to_string(),
            output_json: self.output_json,
            verbosity,
        })
    }
}

/// The result of verification process
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SchemaVerificationResult {
    pub is_verified: bool,
    pub metadata_source: String,
    pub schema: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub output_json: bool,
    #[serde(skip_serializing, skip_deserializing)]
    pub verbosity: Verbosity,
}

impl SchemaVerificationResult {
    /// Display the result in a fancy format
    pub fn display(&self) -> String {
        format!(
            "\n{} {} against schema {}",
            "Successfully verified metadata in".bright_green().bold(),
            format!("`{}`", &self.metadata_source).bold(),
            format!("`{}`!", &self.schema).bold()
        )
    }

    /// Display the build results in a pretty formatted JSON string.
    pub fn serialize_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}
