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

use std::path::Path;

/// Create a `cargo contract` command
fn cargo_contract<P: AsRef<Path>>(path: P) -> assert_cmd::Command {
    let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.current_dir(path).arg("contract");
    cmd
}

#[test]
fn encode_works() {
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

                #[ink(message, selector = 0xBABABABA)]
                pub fn inc(&mut self, by: i32) {
                    self.value.saturating_add(by);
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

    // cargo contract new decode_test
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

    // when
    let output: &str = r#"Encoded data: BABABABA05000000"#;

    // then
    // message selector and data are being encoded properly
    cargo_contract(&project_dir)
        .arg("encode")
        .arg("--message")
        .arg("inc")
        .arg("--args")
        .arg("5")
        .assert()
        .success()
        .stdout(predicates::str::contains(output));

    // when
    let output: &str = r#"Encoded data: CACACACA"#;

    // then
    // message selector is being encoded properly
    cargo_contract(&project_dir)
        .arg("encode")
        .arg("--message")
        .arg("get")
        .assert()
        .success()
        .stdout(predicates::str::contains(output));
}
