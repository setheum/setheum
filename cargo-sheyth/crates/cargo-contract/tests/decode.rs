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
fn decode_works() {
    // given
    let contract = r#"
        #![cfg_attr(not(feature = "std"), no_std, no_main)]

		#[ink::contract]
		mod switcher {
			#[ink(event)]
			pub struct Switched {
				new_value: bool,
			}

			#[ink(storage)]
			pub struct Switcher {
				value: bool,
			}

			impl Switcher {
				#[ink(constructor, selector = 0xBABEBABE)]
				pub fn new(init_value: bool) -> Self {
					Self { value: init_value }
				}

				#[ink(message, selector = 0xBABEBABE)]
				pub fn switch(&mut self, value: bool) {
					self.value = value;
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
        .arg("switcher")
        .assert()
        .success();

    let project_dir = tmp_dir.path().to_path_buf().join("switcher");

    let lib = project_dir.join("lib.rs");
    std::fs::write(lib, contract).expect("Failed to write contract lib.rs");

    tracing::debug!("Building contract in {}", project_dir.to_string_lossy());
    cargo_contract(&project_dir).arg("build").assert().success();

    // when
    let msg_data: &str = "babebabe01";
    let msg_decoded: &str = r#"switch { value: true }"#;

    // then
    // message data is being decoded properly
    cargo_contract(&project_dir)
        .arg("decode")
        .arg("message")
        .arg("--data")
        .arg(msg_data)
        .assert()
        .success()
        .stdout(predicates::str::contains(msg_decoded));

    // and when
    let wrong_msg_data: &str = "babebabe010A";
    let error_msg: &str = "input length was longer than expected by 1 byte(s).\nManaged to decode `switch`, `value` but `0A` bytes were left unread";

    // then
    // wrong message data is being handled properly
    cargo_contract(&project_dir)
        .arg("decode")
        .arg("message")
        .arg("--data")
        .arg(wrong_msg_data)
        .assert()
        .failure()
        .stderr(predicates::str::contains(error_msg));

    // when
    let signature_topic =
        "325c98ff66bd0d9d1c10789ae1f2a17bdfb2dcf6aa3d8092669afafdef1cb72d";
    let event_data: &str = "0001";
    let event_decoded: &str = r#"Switched { new_value: true }"#;

    // then
    // event data is being decoded properly
    cargo_contract(&project_dir)
        .arg("decode")
        .arg("event")
        .arg("--signature-topic")
        .arg(signature_topic)
        .arg("--data")
        .arg(event_data)
        .assert()
        .success()
        .stdout(predicates::str::contains(event_decoded));

    // and when
    let wrong_event_data: &str = "00010C";
    let error_msg: &str = "input length was longer than expected by 1 byte(s).\nManaged to decode `Switched`, `new_value` but `0C` bytes were left unread";

    // then
    // wrong event data is being handled properly
    cargo_contract(&project_dir)
        .arg("decode")
        .arg("event")
        .arg("--signature-topic")
        .arg(signature_topic)
        .arg("--data")
        .arg(wrong_event_data)
        .assert()
        .failure()
        .stderr(predicates::str::contains(error_msg));

    // when
    let constructor_data: &str = "babebabe00";
    let constructor_decoded: &str = r#"new { init_value: false }"#;

    // then
    // constructor data is being decoded properly
    cargo_contract(&project_dir)
        .arg("decode")
        .arg("constructor")
        .arg("-d")
        .arg(constructor_data)
        .assert()
        .success()
        .stdout(predicates::str::contains(constructor_decoded));

    // and when
    let wrong_constructor_data: &str = "babebabe00AC";
    let error_msg: &str = "input length was longer than expected by 1 byte(s).\nManaged to decode `new`, `init_value` but `AC` bytes were left unread";

    // then
    // wrong constructor data is being handled properly
    cargo_contract(&project_dir)
        .arg("decode")
        .arg("constructor")
        .arg("-d")
        .arg(wrong_constructor_data)
        .assert()
        .failure()
        .stderr(predicates::str::contains(error_msg));
}
