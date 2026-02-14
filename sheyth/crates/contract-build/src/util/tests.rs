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

use crate::ManifestPath;
use anyhow::{
    Context,
    Result,
};
use std::{
    fs,
    ops::Deref,
    path::{
        Path,
        PathBuf,
    },
};
use toml::value;

/// Creates a temporary directory and passes the `tmp_dir` path to `f`.
/// Panics if `f` returns an `Err`.
pub fn with_tmp_dir<F>(f: F)
where
    F: FnOnce(&Path) -> Result<()>,
{
    let tmp_dir = tempfile::Builder::new()
        .prefix("cargo-contract.test.")
        .tempdir()
        .expect("temporary directory creation failed");

    // catch test panics in order to clean up temp dir which will be very large
    f(&tmp_dir.path().canonicalize().unwrap()).expect("Error executing test with tmp dir")
}

/// Creates a new contract into a temporary directory. The contract's
/// `ManifestPath` is passed into `f`.
pub fn with_new_contract_project<F>(f: F)
where
    F: FnOnce(ManifestPath) -> Result<()>,
{
    with_tmp_dir(|tmp_dir| {
        let project_name = "new_project";
        crate::new_contract_project(project_name, Some(tmp_dir))
            .expect("new project creation failed");
        let working_dir = tmp_dir.join(project_name);
        let manifest_path = ManifestPath::new(working_dir.join("Cargo.toml"))?;

        f(manifest_path)
    })
}

/// Deletes the mocked executable on `Drop`.
pub struct MockGuard(PathBuf);

impl Drop for MockGuard {
    fn drop(&mut self) {
        std::fs::remove_file(&self.0).ok();
    }
}

impl Deref for MockGuard {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Creates an executable file at `path` with the content `content`.
///
/// Currently works only on `unix`.
#[cfg(unix)]
pub fn create_executable(path: &Path, content: &str) -> MockGuard {
    use std::{
        env,
        io::Write,
        os::unix::fs::PermissionsExt,
    };
    let mut guard = MockGuard(path.to_path_buf());
    let mut file = std::fs::File::create(path).unwrap();
    let path = path.canonicalize().unwrap();
    guard.0.clone_from(&path);
    file.write_all(content.as_bytes())
        .expect("writing of executable failed");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o777))
        .expect("setting permissions failed");

    // make sure the mocked executable is in the path
    let env_paths = {
        let work_dir = path.parent().unwrap().to_path_buf();
        let pathes = env::var_os("PATH").unwrap_or_default();
        let mut pathes: Vec<_> = env::split_paths(&pathes).collect();
        if !pathes.contains(&work_dir) {
            pathes.insert(0, work_dir);
        }
        pathes
    };
    env::set_var("PATH", env::join_paths(env_paths).unwrap());
    guard
}

/// Modify a contracts `Cargo.toml` for testing purposes
pub struct TestContractManifest {
    toml: value::Table,
    manifest_path: ManifestPath,
}

impl TestContractManifest {
    pub fn new(manifest_path: ManifestPath) -> Result<Self> {
        Ok(Self {
            toml: toml::from_str(&fs::read_to_string(&manifest_path)?)?,
            manifest_path,
        })
    }

    fn package_mut(&mut self) -> Result<&mut value::Table> {
        self.toml
            .get_mut("package")
            .context("package section not found")?
            .as_table_mut()
            .context("package section should be a table")
    }

    /// Add a key/value to the `[package.metadata.contract.user]` section
    pub fn add_user_metadata_value(
        &mut self,
        key: &'static str,
        value: value::Value,
    ) -> Result<()> {
        self.package_mut()?
            .entry("metadata")
            .or_insert(value::Value::Table(Default::default()))
            .as_table_mut()
            .context("metadata section should be a table")?
            .entry("contract")
            .or_insert(value::Value::Table(Default::default()))
            .as_table_mut()
            .context("metadata.contract section should be a table")?
            .entry("user")
            .or_insert(value::Value::Table(Default::default()))
            .as_table_mut()
            .context("metadata.contract.user section should be a table")?
            .insert(key.into(), value);
        Ok(())
    }

    pub fn add_package_value(
        &mut self,
        key: &'static str,
        value: value::Value,
    ) -> Result<()> {
        self.package_mut()?.insert(key.into(), value);
        Ok(())
    }

    /// Set `optimization-passes` in `[package.metadata.contract]`
    pub fn set_profile_optimization_passes<P>(
        &mut self,
        passes: P,
    ) -> Result<Option<value::Value>>
    where
        P: ToString,
    {
        Ok(self
            .toml
            .entry("package")
            .or_insert(value::Value::Table(Default::default()))
            .as_table_mut()
            .context("package section should be a table")?
            .entry("metadata")
            .or_insert(value::Value::Table(Default::default()))
            .as_table_mut()
            .context("metadata section should be a table")?
            .entry("contract")
            .or_insert(value::Value::Table(Default::default()))
            .as_table_mut()
            .context("metadata.contract section should be a table")?
            .insert(
                "optimization-passes".to_string(),
                value::Value::String(passes.to_string()),
            ))
    }

    /// Set the dependency version of `package` to `version`.
    pub fn set_dependency_version(
        &mut self,
        dependency: &str,
        version: &str,
    ) -> Result<Option<toml::Value>> {
        Ok(self
            .toml
            .get_mut("dependencies")
            .ok_or_else(|| anyhow::anyhow!("[dependencies] section not found"))?
            .get_mut(dependency)
            .ok_or_else(|| anyhow::anyhow!("{} dependency not found", dependency))?
            .as_table_mut()
            .ok_or_else(|| {
                anyhow::anyhow!("{} dependency should be a table", dependency)
            })?
            .insert("version".into(), value::Value::String(version.into())))
    }

    /// Set the `lib` name to `name`.
    pub fn set_lib_name(&mut self, name: &str) -> Result<Option<toml::Value>> {
        Ok(self
            .toml
            .get_mut("lib")
            .ok_or_else(|| anyhow::anyhow!("[lib] section not found"))?
            .as_table_mut()
            .ok_or_else(|| anyhow::anyhow!("[lib] should be a table"))?
            .insert("name".into(), value::Value::String(name.into())))
    }

    /// Set the `package` name to `name`.
    pub fn set_package_name(&mut self, name: &str) -> Result<Option<toml::Value>> {
        Ok(self
            .toml
            .get_mut("package")
            .ok_or_else(|| anyhow::anyhow!("[package] section not found"))?
            .as_table_mut()
            .ok_or_else(|| anyhow::anyhow!("[package] should be a table"))?
            .insert("name".into(), value::Value::String(name.into())))
    }

    /// Set the `lib` path to `path`.
    pub fn set_lib_path(&mut self, path: &str) -> Result<Option<toml::Value>> {
        Ok(self
            .toml
            .get_mut("lib")
            .ok_or_else(|| anyhow::anyhow!("[lib] section not found"))?
            .as_table_mut()
            .ok_or_else(|| anyhow::anyhow!("[lib] should be a table"))?
            .insert("path".into(), value::Value::String(path.into())))
    }

    pub fn write(&self) -> Result<()> {
        let toml = toml::to_string(&self.toml)?;
        fs::write(&self.manifest_path, toml).map_err(Into::into)
    }
}
