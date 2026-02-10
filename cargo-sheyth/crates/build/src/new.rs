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
use heck::ToUpperCamelCase as _;
use std::{
    env,
    fs,
    io::{
        Cursor,
        Read,
        Seek,
        Write,
    },
    path::{
        Path,
        PathBuf,
    },
};

/// Creates a new contract project from the template.
pub fn new_contract_project<P>(name: &str, dir: Option<P>) -> Result<()>
where
    P: AsRef<Path>,
{
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        anyhow::bail!(
            "Contract names can only contain alphanumeric characters and underscores"
        );
    }

    if !name
        .chars()
        .next()
        .map(|c| c.is_alphabetic())
        .unwrap_or(false)
    {
        anyhow::bail!("Contract names must begin with an alphabetic character");
    }

    let out_dir = dir
        .map_or(env::current_dir()?, |p| p.as_ref().to_path_buf())
        .join(name);
    if out_dir.join("Cargo.toml").exists() {
        anyhow::bail!("A Cargo package already exists in {}", name);
    }
    if !out_dir.exists() {
        fs::create_dir(&out_dir)?;
    }

    let template = include_bytes!(concat!(env!("OUT_DIR"), "/template.zip"));

    unzip(template, out_dir, Some(name))?;

    Ok(())
}

// Unzips the file at `template` to `out_dir`.
//
// In case `name` is set the zip file is treated as if it were a template for a new
// contract. Replacements in `Cargo.toml` for `name`-placeholders are attempted in
// that case.
fn unzip(template: &[u8], out_dir: PathBuf, name: Option<&str>) -> Result<()> {
    let mut cursor = Cursor::new(Vec::new());
    cursor.write_all(template)?;
    cursor.rewind()?;

    let mut archive = zip::ZipArchive::new(cursor)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = out_dir.join(file.name());

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(outpath.clone())
                .map_err(|e| {
                    if e.kind() == std::io::ErrorKind::AlreadyExists {
                        anyhow::anyhow!("File {} already exists", file.name(),)
                    } else {
                        anyhow::anyhow!(e)
                    }
                })?;

            if let Some(name) = name {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                let contents = contents.replace("{{name}}", name);
                let contents =
                    contents.replace("{{camel_name}}", &name.to_upper_camel_case());
                outfile.write_all(contents.as_bytes())?;
            } else {
                let mut v = Vec::new();
                file.read_to_end(&mut v)?;
                outfile.write_all(v.as_slice())?;
            }
        }

        // Get and set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tests::with_tmp_dir;

    #[test]
    fn rejects_hyphenated_name() {
        with_tmp_dir(|path| {
            let result = new_contract_project("rejects-hyphenated-name", Some(path));
            assert!(result.is_err(), "Should fail");
            assert_eq!(
                result.err().unwrap().to_string(),
                "Contract names can only contain alphanumeric characters and underscores"
            );
            Ok(())
        })
    }

    #[test]
    fn rejects_name_with_period() {
        with_tmp_dir(|path| {
            let result = new_contract_project("../xxx", Some(path));
            assert!(result.is_err(), "Should fail");
            assert_eq!(
                result.err().unwrap().to_string(),
                "Contract names can only contain alphanumeric characters and underscores"
            );
            Ok(())
        })
    }

    #[test]
    fn rejects_name_beginning_with_number() {
        with_tmp_dir(|path| {
            let result = new_contract_project("1xxx", Some(path));
            assert!(result.is_err(), "Should fail");
            assert_eq!(
                result.err().unwrap().to_string(),
                "Contract names must begin with an alphabetic character"
            );
            Ok(())
        })
    }

    #[test]
    fn contract_cargo_project_already_exists() {
        with_tmp_dir(|path| {
            let name = "test_contract_cargo_project_already_exists";
            let _ = new_contract_project(name, Some(path));
            let result = new_contract_project(name, Some(path));

            assert!(result.is_err(), "Should fail");
            assert_eq!(
                result.err().unwrap().to_string(),
                "A Cargo package already exists in test_contract_cargo_project_already_exists"
            );
            Ok(())
        })
    }

    #[test]
    fn dont_overwrite_existing_files_not_in_cargo_project() {
        with_tmp_dir(|path| {
            let name = "dont_overwrite_existing_files";
            let dir = path.join(name);
            fs::create_dir_all(&dir).unwrap();
            fs::File::create(dir.join(".gitignore")).unwrap();
            let result = new_contract_project(name, Some(path));

            assert!(result.is_err(), "Should fail");
            assert_eq!(
                result.err().unwrap().to_string(),
                "File .gitignore already exists"
            );
            Ok(())
        })
    }
}
