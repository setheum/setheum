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

use std::str::FromStr;

use contract_transcode::AccountId32;
use subxt::{
    backend::rpc::{
        RawValue,
        RpcClient,
        RpcParams,
    },
    ext::scale_value::{
        stringify::{
            from_str_custom,
            ParseError,
        },
        Value,
    },
};

use crate::url_to_string;
use anyhow::{
    anyhow,
    bail,
    Result,
};

pub struct RawParams(Option<Box<RawValue>>);

impl RawParams {
    /// Creates a new `RawParams` instance from a slice of string parameters.
    /// Returns a `Result` containing the parsed `RawParams` or an error if parsing fails.
    pub fn new(params: &[String]) -> Result<Self> {
        let mut str_parser = from_str_custom();
        str_parser = str_parser.add_custom_parser(custom_hex_parse);
        str_parser = str_parser.add_custom_parser(custom_ss58_parse);

        let value_params = params
            .iter()
            .map(|e| str_parser.parse(e).0)
            .collect::<Result<Vec<_>, ParseError>>()
            .map_err(|e| anyhow::anyhow!("Method parameters parsing failed: {e}"))?;

        let params = match value_params.is_empty() {
            true => None,
            false => {
                value_params
                    .iter()
                    .try_fold(RpcParams::new(), |mut v, e| {
                        v.push(e)?;
                        Ok(v)
                    })
                    .map_err(|e: subxt::Error| {
                        anyhow::anyhow!("Building method parameters failed: {e}")
                    })?
                    .build()
            }
        };

        Ok(Self(params))
    }
}

pub struct RpcRequest(RpcClient);

impl RpcRequest {
    /// Creates a new `RpcRequest` instance.
    pub async fn new(url: &url::Url) -> Result<Self> {
        let rpc = RpcClient::from_url(url_to_string(url)).await?;
        Ok(Self(rpc))
    }

    /// Performs a raw RPC call with the specified method and parameters.
    /// Returns a `Result` containing the raw RPC call result or an error if the call
    /// fails.
    pub async fn raw_call<'a>(
        &'a self,
        method: &'a str,
        params: RawParams,
    ) -> Result<Box<RawValue>> {
        let methods = self.get_supported_methods().await?;
        if !methods.iter().any(|e| e == method) {
            bail!(
                "Method not found, supported methods: {}",
                methods.join(", ")
            );
        }
        self.0
            .request_raw(method, params.0)
            .await
            .map_err(|e| anyhow!("Raw RPC call failed: {e}"))
    }

    /// Retrieves the supported RPC methods.
    /// Returns a `Result` containing a vector of supported RPC methods or an error if the
    /// call fails.
    async fn get_supported_methods(&self) -> Result<Vec<String>> {
        let result = self
            .0
            .request_raw("rpc_methods", None)
            .await
            .map_err(|e| anyhow!("Rpc call 'rpc_methods' failed: {e}"))?;

        let result_value: serde_json::Value = serde_json::from_str(result.get())?;

        let methods = result_value
            .get("methods")
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow!("Methods field parsing failed!"))?;

        // Exclude unupported methods using pattern matching
        let patterns = ["watch", "unstable", "subscribe"];
        let filtered_methods: Vec<String> = methods
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .filter(|s| {
                patterns
                    .iter()
                    .all(|&pattern| !s.to_lowercase().contains(pattern))
            })
            .collect();

        Ok(filtered_methods)
    }
}

/// Parse hex to string
fn custom_hex_parse(s: &mut &str) -> Option<Result<Value<()>, ParseError>> {
    if !s.starts_with("0x") {
        return None
    }

    let end_idx = s
        .find(|c: char| !c.is_ascii_alphanumeric())
        .unwrap_or(s.len());
    let hex = &s[..end_idx];
    *s = &s[end_idx..];
    Some(Ok(Value::string(hex.to_string())))
}

/// Parse ss58 address to string
fn custom_ss58_parse(s: &mut &str) -> Option<Result<Value<()>, ParseError>> {
    let end_idx = s
        .find(|c: char| !c.is_ascii_alphanumeric())
        .unwrap_or(s.len());
    let account = AccountId32::from_str(&s[..end_idx]).ok()?;

    *s = &s[end_idx..];
    Some(Ok(Value::string(format!("0x{}", hex::encode(account.0)))))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn assert_raw_params_value(input: &[&str], expected: &str) {
        let input = input.iter().map(|e| e.to_string()).collect::<Vec<String>>();
        let raw_params = RawParams::new(&input).expect("Raw param shall be created");
        let expected = expected
            .chars()
            .filter(|&c| !c.is_whitespace())
            .collect::<String>();
        assert_eq!(raw_params.0.unwrap().get(), expected);
    }

    #[test]
    fn parse_ss58_works() {
        let expected = r#"["0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","sr25"]"#;
        let input = &[
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
            "\"sr25\"",
        ];
        assert_raw_params_value(input, expected);
    }

    #[test]
    fn parse_seq_works() {
        let expected = r#"[[1,"0x1234",true]]"#;
        let input = &["(1, 0x1234, true)"];
        assert_raw_params_value(input, expected);
    }

    #[test]
    fn parse_map_works() {
        let expected = r#"[{
            "hello": true,
            "a": 4,
            "b": "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
            "c": "test"
        }]"#;
        let input = &["{hello: true, a: 4, b: \
        5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY, c: \"test\"}"];
        assert_raw_params_value(input, expected);
    }
}
