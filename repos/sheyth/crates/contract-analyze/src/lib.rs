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

#![deny(unused_crate_dependencies)]

use anyhow::{
    anyhow,
    bail,
    Result,
};
pub use contract_metadata::Language;
use std::collections::HashMap;
use wasmparser::{
    BinaryReader,
    FuncType,
    Import,
    Name,
    NameSectionReader,
    Operator,
    Parser,
    Payload,
    TypeRef,
    ValType,
};

/// WebAssembly module
#[derive(Default)]
pub struct Module<'a> {
    /// Map the custom section name to its data.
    pub custom_sections: HashMap<&'a str, &'a [u8]>,
    /// Start section function.
    pub start_section: Option<u32>,
    /// Map the function index to the type index.
    pub function_sections: Vec<u32>,
    /// Type sections containing functions only.
    pub type_sections: Vec<FuncType>,
    /// Import sections.
    pub import_sections: Vec<Import<'a>>,
    /// Code sections containing instructions only.
    pub code_sections: Vec<Vec<Operator<'a>>>,
}

impl<'a> Module<'a> {
    /// Parse the Wasm module.
    fn parse(code: &'a [u8]) -> Result<Self> {
        let mut module: Module<'a> = Default::default();
        for payload in Parser::new(0).parse_all(code) {
            let payload = payload?;

            match payload {
                Payload::Version {
                    num: _,
                    encoding: wasmparser::Encoding::Component,
                    range: _,
                } => {
                    anyhow::bail!("Unsupported component section.")
                }
                Payload::End(_) => break,
                Payload::CustomSection(ref c) => {
                    module.custom_sections.insert(c.name(), c.data());
                }
                Payload::StartSection { func, range: _ } => {
                    module.start_section = Some(func);
                }
                Payload::CodeSectionStart {
                    count: _,
                    range,
                    size: _,
                } => {
                    let binary_reader = BinaryReader::new(&code[range], 0);
                    let reader = wasmparser::CodeSectionReader::new(binary_reader)?;
                    for body in reader {
                        let body = body?;
                        let reader = body.get_operators_reader();
                        let operators = reader?;
                        let ops = operators
                            .into_iter()
                            .collect::<std::result::Result<Vec<_>, _>>()?;
                        module.code_sections.push(ops);
                    }
                }
                Payload::ImportSection(reader) => {
                    for ty in reader {
                        module.import_sections.push(ty?);
                    }
                }
                Payload::TypeSection(reader) => {
                    // Save function types
                    for ty in reader.into_iter_err_on_gc_types() {
                        module.type_sections.push(ty?);
                    }
                }
                Payload::FunctionSection(reader) => {
                    for ty in reader {
                        module.function_sections.push(ty?);
                    }
                }
                _ => {}
            }
        }
        Ok(module)
    }

    /// Create a Module from the Wasm code.
    pub fn new(code: &'a [u8]) -> Result<Self> {
        Self::parse(code)
    }

    /// Check if the function name is present in the 'name' custom section.
    pub fn has_function_name(&self, name: &str) -> Result<bool> {
        // The contract compiled in debug mode includes function names in the name
        // section.
        let name_section = self
            .custom_sections
            .get("name")
            .ok_or(anyhow!("Custom section 'name' not found."))?;
        let binary_reader = BinaryReader::new(name_section, 0);
        let reader = NameSectionReader::new(binary_reader);
        for section in reader {
            if let Name::Function(name_reader) = section? {
                for naming in name_reader {
                    let naming = naming?;
                    if naming.name.contains(name) {
                        return Ok(true)
                    }
                }
            }
        }
        Ok(false)
    }

    /// Get the function's type index from the type section.
    pub fn function_type_index(&self, function: &FuncType) -> Option<usize> {
        self.type_sections.iter().enumerate().find_map(|(i, ty)| {
            if ty == function {
                return Some(i)
            }
            None
        })
    }

    /// Get the function index from the import section.
    pub fn function_import_index(&self, name: &str) -> Option<usize> {
        self.import_sections
            .iter()
            .filter(|&entry| matches!(entry.ty, TypeRef::Func(_)))
            .position(|e| e.name == name)
    }

    /// Search for a functions in a WebAssembly (Wasm) module that matches a given
    /// function type.
    ///
    /// If one or more functions matching the specified type are found, this function
    /// returns their bodies in a vector; otherwise, it returns an error.
    pub fn functions_by_type(
        &self,
        function_type: &FuncType,
    ) -> Result<Vec<Vec<Operator>>> {
        self.function_sections
            .iter()
            .enumerate()
            .filter(|(_, &elem)| {
                Some(elem as usize) == self.function_type_index(function_type)
            })
            .map(|(index, _)| {
                self.code_sections
                    .get(index)
                    .ok_or(anyhow!("Requested function not found in code section."))
                    .cloned()
            })
            .collect::<Result<Vec<_>>>()
    }
}

/// Checks if a ink! function is present.
fn is_ink_function_present(module: &Module) -> bool {
    // Signature for 'deny_payment' ink! function.
    let ink_func_deny_payment_sig = FuncType::new(vec![], vec![ValType::I32]);
    // Signature for 'transferred_value' ink! function.
    let ink_func_transferred_value_sig = FuncType::new(vec![ValType::I32], vec![]);

    // The deny_payment and transferred_value functions internally call the
    // value_transferred function. Getting its index from import section.
    let value_transferred_index =
        // For ink! >=4
        module.function_import_index("value_transferred").or(
            // For ink! ^3
            module.function_import_index("seal_value_transferred"),
        );

    let mut functions: Vec<Vec<Operator>> = Vec::new();
    let function_signatures =
        vec![&ink_func_deny_payment_sig, &ink_func_transferred_value_sig];

    for signature in function_signatures {
        if let Ok(mut func) = module.functions_by_type(signature) {
            functions.append(&mut func);
        }
    }
    if let Some(index) = value_transferred_index {
        functions.iter().any(|body| {
        body.iter().any(|instruction| {
            // Matches the 'value_transferred' function.
            matches!(instruction, &Operator::Call{function_index} if function_index as usize == index)
        })
    })
    } else {
        false
    }
}

/// Detects the programming language of a smart contract from its WebAssembly (Wasm)
/// binary code.
///
/// This function accepts a Wasm code as input and employs a set of heuristics to identify
/// the contract's source language. It currently supports detection for Ink!, Solidity,
/// and AssemblyScript languages.
pub fn determine_language(code: &[u8]) -> Result<Language> {
    let module = Module::new(code)?;
    let start_section = module.start_section.is_some();

    if !start_section && module.custom_sections.keys().any(|e| e == &"producers") {
        return Ok(Language::Solidity)
    } else if start_section
        && module
            .custom_sections
            .keys()
            .any(|e| e == &"sourceMappingURL")
    {
        return Ok(Language::AssemblyScript)
    } else if !start_section
        && (is_ink_function_present(&module)
            || matches!(module.has_function_name("ink_env"), Ok(true)))
    {
        return Ok(Language::Ink)
    }

    bail!("Language unsupported or unrecognized.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failes_with_unsupported_language() {
        let contract = r#"
        (module
            (type $none_=>_none (func))
            (type (;0;) (func (param i32 i32 i32)))
            (import "env" "memory" (func (;5;) (type 0)))
            (start $~start)
            (func $~start (type $none_=>_none))
            (func (;5;) (type 0))
        )
        "#;
        let code = &wat::parse_str(contract).expect("Invalid wat.");
        let lang = determine_language(code);
        assert!(lang.is_err());
        assert_eq!(
            lang.unwrap_err().to_string(),
            "Language unsupported or unrecognized."
        );
    }

    #[test]
    fn determines_ink_language() {
        let contract = r#"
        (module
            (type (;0;) (func (param i32 i32 i32)))
            (type (;1;) (func (result i32)))
            (type (;2;) (func (param i32 i32)))
            (import "seal" "foo" (func (;0;) (type 0)))
            (import "seal0" "value_transferred" (func (;1;) (type 2)))
            (import "env" "memory" (memory (;0;) 2 16))
            (func (;2;) (type 2))
            (func (;3;) (type 1) (result i32)
            (local i32 i64 i64)
            global.get 0
            i32.const 32
            i32.sub
            local.tee 0
            global.set 0
            local.get 0
            i64.const 0
            i64.store offset=8
            local.get 0
            i64.const 0
            i64.store
            local.get 0
            i32.const 16
            i32.store offset=28
            local.get 0
            local.get 0
            i32.const 28
            i32.add
            call 1
            local.get 0
            i64.load offset=8
            local.set 1
            local.get 0
            i64.load
            local.set 2
            local.get 0
            i32.const 32
            i32.add
            global.set 0
            i32.const 5
            i32.const 4
            local.get 1
            local.get 2
            i64.or
            i64.eqz
            select
        )
            (global (;0;) (mut i32) (i32.const 65536))
        )"#;
        let code = &wat::parse_str(contract).expect("Invalid wat.");
        let lang = determine_language(code);
        assert!(
            matches!(lang, Ok(Language::Ink)),
            "Failed to detect Ink! language."
        );
    }

    #[test]
    fn determines_solidity_language() {
        let contract = r#"
        (module
            (type (;0;) (func (param i32 i32 i32)))
            (import "env" "memory" (memory (;0;) 16 16))
            (func (;0;) (type 0))
            (@custom "producers" "data")
        )
        "#;
        let code = &wat::parse_str(contract).expect("Invalid wat.");
        let lang = determine_language(code);
        assert!(
            matches!(lang, Ok(Language::Solidity)),
            "Failed to detect Solidity language."
        );
    }

    #[test]
    fn determines_assembly_script_language() {
        let contract = r#"
        (module
            (type $none_=>_none (func))
            (type (;0;) (func (param i32 i32 i32)))
            (import "seal" "foo" (func (;0;) (type 0)))
            (import "env" "memory" (memory $0 2 16))
            (start $~start)
            (func $~start (type $none_=>_none))
            (func (;1;) (type 0))
            (@custom "sourceMappingURL" "data")
        )
        "#;
        let code = &wat::parse_str(contract).expect("Invalid wat.");
        let lang = determine_language(code);
        assert!(
            matches!(lang, Ok(Language::AssemblyScript)),
            "Failed to detect AssemblyScript language."
        );
    }
}
