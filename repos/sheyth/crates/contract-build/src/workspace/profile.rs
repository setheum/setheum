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

use toml::value;

/// Subset of cargo profile settings to configure defaults for building contracts.
///
/// All fields are optional, and if not set, the default value from cargo will be used.
/// See https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles.
#[derive(Default)]
pub struct Profile {
    pub opt_level: Option<OptLevel>,
    pub lto: Option<Lto>,
    // `None` means use rustc default.
    pub codegen_units: Option<u32>,
    pub panic: Option<PanicStrategy>,
}

impl Profile {
    /// The preferred set of defaults for compiling a release build of a contract
    pub fn default_contract_release() -> Profile {
        Profile {
            opt_level: Some(OptLevel::Z),
            lto: Some(Lto::Fat),
            codegen_units: Some(1),
            panic: Some(PanicStrategy::Abort),
        }
    }

    /// Set any unset profile settings from the config.
    ///
    /// Therefore:
    ///   - If the user has explicitly defined a profile setting, it will not be
    ///     overwritten.
    ///   - If a profile setting is not defined, the value from this profile instance will
    ///     be added
    pub(super) fn merge(&self, profile: &mut value::Table) {
        fn set_value_if_vacant<T>(
            key: &'static str,
            value: Option<T>,
            profile: &mut value::Table,
        ) where
            T: Into<value::Value>,
        {
            if let Some(value) = value {
                if !profile.contains_key(key) {
                    profile.insert(key.into(), value.into());
                }
            }
        }
        set_value_if_vacant(
            "opt-level",
            self.opt_level.map(OptLevel::to_toml_value),
            profile,
        );
        set_value_if_vacant("lto", self.lto.map(Lto::to_toml_value), profile);
        set_value_if_vacant("codegen-units", self.codegen_units, profile);
        set_value_if_vacant(
            "panic",
            self.panic.map(PanicStrategy::to_toml_value),
            profile,
        );
    }
}

/// The [`opt-level`](https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level) setting
#[allow(unused)]
#[derive(Clone, Copy)]
pub enum OptLevel {
    NoOptimizations,
    O1,
    O2,
    O3,
    S,
    Z,
}

impl OptLevel {
    fn to_toml_value(self) -> value::Value {
        match self {
            OptLevel::NoOptimizations => 0.into(),
            OptLevel::O1 => 1.into(),
            OptLevel::O2 => 2.into(),
            OptLevel::O3 => 3.into(),
            OptLevel::S => "s".into(),
            OptLevel::Z => "z".into(),
        }
    }
}

/// The [`link-time-optimization`](https://doc.rust-lang.org/cargo/reference/profiles.html#lto) setting.
#[derive(Clone, Copy)]
#[allow(unused)]
pub enum Lto {
    /// Sets `lto = false`
    ThinLocal,
    /// Sets `lto = "fat"`, the equivalent of `lto = true`
    Fat,
    /// Sets `lto = "thin"`
    Thin,
    /// Sets `lto = "off"`
    Off,
}

impl Lto {
    fn to_toml_value(self) -> value::Value {
        match self {
            Lto::ThinLocal => false.into(),
            Lto::Fat => "fat".into(),
            Lto::Thin => "thin".into(),
            Lto::Off => "off".into(),
        }
    }
}

/// The `panic` setting.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
#[allow(unused)]
pub enum PanicStrategy {
    Unwind,
    Abort,
}

impl PanicStrategy {
    fn to_toml_value(self) -> value::Value {
        match self {
            PanicStrategy::Unwind => "unwind".into(),
            PanicStrategy::Abort => "abort".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn merge_profile_inserts_preferred_defaults() {
        let profile = Profile::default_contract_release();

        // no `[profile.release]` section specified
        let manifest_toml = "";
        let mut expected = value::Table::new();
        expected.insert("opt-level".into(), value::Value::String("z".into()));
        expected.insert("lto".into(), value::Value::String("fat".into()));
        expected.insert("codegen-units".into(), value::Value::Integer(1));
        expected.insert("panic".into(), value::Value::String("abort".into()));

        let mut manifest_profile = toml::from_str(manifest_toml).unwrap();

        profile.merge(&mut manifest_profile);

        assert_eq!(expected, manifest_profile)
    }

    #[test]
    fn merge_profile_preserves_user_defined_settings() {
        let profile = Profile::default_contract_release();

        let manifest_toml = r#"
            panic = "unwind"
            lto = false
            opt-level = 3
            codegen-units = 256
        "#;
        let mut expected = value::Table::new();
        expected.insert("opt-level".into(), value::Value::Integer(3));
        expected.insert("lto".into(), value::Value::Boolean(false));
        expected.insert("codegen-units".into(), value::Value::Integer(256));
        expected.insert("panic".into(), value::Value::String("unwind".into()));

        let mut manifest_profile = toml::from_str(manifest_toml).unwrap();

        profile.merge(&mut manifest_profile);

        assert_eq!(expected, manifest_profile)
    }
}
