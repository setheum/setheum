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

use serde_json::{
    Map,
    Value,
};
use std::path::{
    Path,
    PathBuf,
};
use tempfile::TempDir;

/// Create a `cargo contract` command
fn cargo_contract<P: AsRef<Path>>(path: P) -> assert_cmd::Command {
    let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.current_dir(path).arg("contract");
    cmd
}

/// Compile the reference contract and return a byte array of its bundle and raw wasm
/// binary.
fn compile_reference_contract() -> (Vec<u8>, Vec<u8>) {
    let contract = r#"
    #![cfg_attr(not(feature = "std"), no_std, no_main)]

    #[ink::contract]
    mod incrementer {
        #[ink(storage)]
        pub struct Incrementer {
            value: i32,
        }

        impl Incrementer {
            #[ink(constructor)]
            pub fn new(init_value: i32) -> Self {
                Self { value: init_value }
            }

            #[ink(constructor)]
            pub fn new_default() -> Self {
                Self::new(Default::default())
            }

            #[ink(message)]
            pub fn inc(&mut self, by: i32) {
                self.value = self.value.saturating_add(by);
            }

            #[ink(message, selector = 0xCACACACA)]
            pub fn get(&self) -> i32 {
                self.value
            }
        }
    }"#;
    let tmp_dir = tempfile::Builder::new()
        .prefix("cargo-contract.cli.test.")
        .tempdir()
        .expect("temporary directory creation failed");

    // cargo contract new reference contract
    cargo_contract(tmp_dir.path())
        .arg("new")
        .arg("incrementer")
        .assert()
        .success();

    let project_dir = tmp_dir.path().to_path_buf().join("incrementer");

    let lib = project_dir.join("lib.rs");
    std::fs::write(lib, contract).expect("Failed to write contract lib.rs");

    tracing::debug!("Building contract in {}", project_dir.to_string_lossy());
    cargo_contract(&project_dir)
        .arg("build")
        .arg("--release")
        .assert()
        .success();

    let bundle_path = project_dir.join("target/ink/incrementer.contract");
    let bundle = std::fs::read(bundle_path)
        .expect("Failed to read the content of the contract bundle!");

    let wasm_path = project_dir.join("target/ink/incrementer.wasm");
    let wasm = std::fs::read(wasm_path)
        .expect("Failed to read the content of the contract binary!");

    (bundle, wasm)
}

#[test]
fn verify_equivalent_contracts() {
    // given
    let contract = r#"
    #![cfg_attr(not(feature = "std"), no_std, no_main)]

    #[ink::contract]
    mod incrementer {
        #[ink(storage)]
        pub struct Incrementer {
            value: i32,
        }

        impl Incrementer {
            #[ink(constructor)]
            pub fn new(init_value: i32) -> Self {
                Self { value: init_value }
            }

            #[ink(constructor)]
            pub fn new_default() -> Self {
                Self::new(Default::default())
            }

            #[ink(message)]
            pub fn inc(&mut self, by: i32) {
                self.value = self.value.saturating_add(by);
            }

            #[ink(message, selector = 0xCACACACA)]
            pub fn get(&self) -> i32 {
                self.value
            }
        }
    }"#;
    let tmp_dir = tempfile::Builder::new()
        .prefix("cargo-contract.cli.test.")
        .tempdir()
        .expect("temporary directory creation failed");

    // cargo contract new sample contract
    cargo_contract(tmp_dir.path())
        .arg("new")
        .arg("incrementer")
        .assert()
        .success();

    let project_dir = tmp_dir.path().to_path_buf().join("incrementer");

    let lib = project_dir.join("lib.rs");
    std::fs::write(lib, contract).expect("Failed to write contract lib.rs");

    // Compile reference contract and write bundle and wasm in the directory.
    let (ref_bundle, ref_wasm) = compile_reference_contract();
    let bundle = project_dir.join("reference.contract");
    std::fs::write(bundle, ref_bundle)
        .expect("Failed to write bundle contract to the current dir!");
    let wasm = project_dir.join("reference.wasm");
    std::fs::write(wasm, ref_wasm)
        .expect("Failed to write wasm binary to the current dir!");

    // when
    let output: &str = r#""is_verified": true"#;

    // then
    cargo_contract(&project_dir)
        .arg("verify")
        .arg("--contract")
        .arg("reference.contract")
        .arg("--output-json")
        .assert()
        .success()
        .stdout(predicates::str::contains(output));
    // and
    cargo_contract(&project_dir)
        .arg("verify")
        .arg("--wasm")
        .arg("reference.wasm")
        .arg("--output-json")
        .assert()
        .success()
        .stdout(predicates::str::contains(output));
}

#[test]
fn verify_different_contracts() {
    // given
    let contract = r#"
    #![cfg_attr(not(feature = "std"), no_std, no_main)]

    #[ink::contract]
    mod incrementer {
        #[ink(storage)]
        pub struct Incrementer {
            value: i32,
        }

        impl Incrementer {
            #[ink(constructor)]
            pub fn new(init_value: i32) -> Self {
                Self { value: init_value }
            }

            #[ink(constructor)]
            pub fn new_default() -> Self {
                Self::new(Default::default())
            }

            #[ink(message)]
            pub fn inc(&mut self, by: i32) {
                self.value = self.value.saturating_add(by);
            }

            #[ink(message, selector = 0xCBCBCBCB)]
            pub fn get(&self) -> i32 {
                self.value
            }
        }
    }"#;

    let tmp_dir = tempfile::Builder::new()
        .prefix("cargo-contract.cli.test.")
        .tempdir()
        .expect("temporary directory creation failed");

    // cargo contract new sample contract.
    cargo_contract(tmp_dir.path())
        .arg("new")
        .arg("incrementer")
        .assert()
        .success();

    let project_dir = tmp_dir.path().to_path_buf().join("incrementer");

    let lib = project_dir.join("lib.rs");
    std::fs::write(lib, contract).expect("Failed to write contract lib.rs");

    tracing::debug!("Building contract in {}", project_dir.to_string_lossy());
    cargo_contract(&project_dir).arg("build").assert().success();

    // Compile reference contract and write bundle and wasm in the directory.
    let (ref_bundle, ref_wasm) = compile_reference_contract();
    let bundle = project_dir.join("reference.contract");
    std::fs::write(bundle, ref_bundle)
        .expect("Failed to write bundle contract to the current dir!");
    let wasm = project_dir.join("reference.wasm");
    std::fs::write(wasm, ref_wasm)
        .expect("Failed to write wasm binary to the current dir!");

    // when
    let output: &str = "Failed to verify `reference.contract` against the workspace at \
                        `Cargo.toml`: the hashed Wasm blobs are not matching.";

    // then
    cargo_contract(&project_dir)
        .arg("verify")
        .arg("--contract")
        .arg("reference.contract")
        .arg("--output-json")
        .assert()
        .failure()
        .stderr(predicates::str::contains(output));
    // and

    let output: &str =
        r#"Failed to verify the authenticity of wasm binary at `reference.wasm`"#;
    cargo_contract(&project_dir)
        .arg("verify")
        .arg("--wasm")
        .arg("reference.wasm")
        .arg("--output-json")
        .assert()
        .failure()
        .stderr(predicates::str::contains(output));
}

#[test]
fn verify_must_fail_on_manipulated_wasm_code() {
    // given
    let tmp_dir = tempfile::Builder::new()
        .prefix("cargo-contract.cli.test.")
        .tempdir()
        .expect("temporary directory creation failed");
    let (project_dir, mut metadata_json) = create_and_compile_minimal_contract(&tmp_dir);

    // when
    // we change the `source.wasm` blob to a different contract bytecode, but the hash
    // will remain the same as the one from our compiled minimal contract.
    let source = metadata_json
        .get_mut("source")
        .expect("source field not found in metadata");
    let wasm = source
        .get_mut("wasm")
        .expect("source.wasm field not found in metadata");
    *wasm = Value::String(String::from("0x00"));

    let contract_file =
        project_dir.join("contract_with_mismatching_wasm_hash_and_code.contract");
    let metadata = serde_json::to_string_pretty(&metadata_json)
        .expect("failed converting metadata to json");
    std::fs::write(contract_file, metadata)
        .expect("Failed to write bundle contract to the current dir!");

    // then
    let output: &str = "Failed to verify `contract_with_mismatching_wasm_hash_and_code.contract` \
                        against the workspace at `Cargo.toml`: the hashed Wasm blobs are not \
                        matching.";
    cargo_contract(&project_dir)
        .arg("verify")
        .arg("--contract")
        .arg("contract_with_mismatching_wasm_hash_and_code.contract")
        .arg("--output-json")
        .assert()
        .failure()
        .stderr(predicates::str::contains(output));
}

#[test]
fn verify_must_fail_on_corrupt_hash() {
    // given
    let tmp_dir = tempfile::Builder::new()
        .prefix("cargo-contract.cli.test.")
        .tempdir()
        .expect("temporary directory creation failed");
    let (project_dir, mut metadata_json) = create_and_compile_minimal_contract(&tmp_dir);

    // when
    // we change the `source.hash` value to a different hash
    let source = metadata_json
        .get_mut("source")
        .expect("source field not found in metadata");
    let wasm = source
        .get_mut("hash")
        .expect("source.hash field not found in metadata");
    *wasm = Value::String(String::from(
        "0x0000000000000000000000000000000000000000000000000000000000000000",
    ));

    let contract_file = project_dir.join("contract_with_corrupt_hash.contract");
    let metadata = serde_json::to_string_pretty(&metadata_json)
        .expect("failed converting metadata to json");
    std::fs::write(contract_file, metadata)
        .expect("Failed to write bundle contract to the current dir!");

    // then
    let output: &str = "The reference contract `contract_with_corrupt_hash.contract` \
                        metadata is corrupt: the source.hash does not match the source.wasm hash.";
    cargo_contract(&project_dir)
        .arg("verify")
        .arg("--contract")
        .arg("contract_with_corrupt_hash.contract")
        .arg("--output-json")
        .assert()
        .failure()
        .stderr(predicates::str::contains(output));
}

// Creates a minimal contract in `tmp_dir` and compiles it.
//
// Returns a tuple of:
//  * the workspace folder within `tmp_dir` and
//  * the metadata contained in the `.contract` file that build.
fn create_and_compile_minimal_contract(
    tmp_dir: &TempDir,
) -> (PathBuf, Map<String, Value>) {
    let contract = r#"
    #![cfg_attr(not(feature = "std"), no_std, no_main)]

    #[ink::contract]
    mod minimal {
        #[ink(storage)]
        pub struct Minimal {}

        impl Minimal {
            #[ink(constructor)]
            pub fn new() -> Self {
                Self { }
            }

            #[ink(message)]
            pub fn void(&self) { }
        }
    }"#;
    cargo_contract(tmp_dir.path())
        .arg("new")
        .arg("minimal")
        .assert()
        .success();
    let project_dir = tmp_dir.path().to_path_buf().join("minimal");
    let lib = project_dir.join("lib.rs");
    std::fs::write(lib, contract).expect("Failed to write contract lib.rs");

    tracing::debug!("Building contract in {}", project_dir.to_string_lossy());
    cargo_contract(&project_dir)
        .arg("build")
        .arg("--release")
        .assert()
        .success();

    let bundle_path = project_dir.join("target/ink/minimal.contract");
    let bundle = std::fs::read(bundle_path)
        .expect("Failed to read the content of the contract bundle!");
    let metadata_json: Map<String, Value> = serde_json::from_slice(&bundle).unwrap();

    (project_dir, metadata_json)
}
