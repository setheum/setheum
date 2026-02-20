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
    env,
    ffi::OsStr,
    fs::File,
    io::{
        prelude::*,
        Write,
    },
    iter::Iterator,
    path::{
        Path,
        PathBuf,
    },
};

use anyhow::Result;
use walkdir::WalkDir;
use zip::{
    write::FileOptions,
    CompressionMethod,
    ZipWriter,
};

const DEFAULT_UNIX_PERMISSIONS: u32 = 0o755;

fn main() {
    let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR should be set by cargo")
        .into();
    let out_dir: PathBuf = env::var("OUT_DIR")
        .expect("OUT_DIR should be set by cargo")
        .into();
    let res = zip_template(&manifest_dir, &out_dir);

    match res {
        Ok(()) => std::process::exit(0),
        Err(err) => {
            eprintln!("Encountered error: {err:?}");
            std::process::exit(1)
        }
    }
}

/// Creates a zip archive `template.zip` of the `new` project template in `out_dir`.
fn zip_template(manifest_dir: &Path, out_dir: &Path) -> Result<()> {
    let template_dir = manifest_dir.join("templates").join("new");
    let template_dst_file = out_dir.join("template.zip");
    println!(
        "Creating template zip: template_dir '{}', destination archive '{}'",
        template_dir.display(),
        template_dst_file.display()
    );
    zip_dir(&template_dir, &template_dst_file, CompressionMethod::Stored).map(|_| {
        println!(
            "Done: {} written to {}",
            template_dir.display(),
            template_dst_file.display()
        );
    })
}

/// Creates a zip archive at `dst_file` with the content of the `src_dir`.
fn zip_dir(src_dir: &Path, dst_file: &Path, method: CompressionMethod) -> Result<()> {
    if !src_dir.exists() {
        anyhow::bail!("src_dir '{}' does not exist", src_dir.display());
    }
    if !src_dir.is_dir() {
        anyhow::bail!("src_dir '{}' is not a directory", src_dir.display());
    }

    let file = File::create(dst_file)?;

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter().filter_map(|e| e.ok());

    let mut zip = ZipWriter::new(file);
    let options = FileOptions::<()>::default()
        .compression_method(method)
        .unix_permissions(DEFAULT_UNIX_PERMISSIONS);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let mut name = path.strip_prefix(src_dir)?.to_path_buf();

        // `Cargo.toml` files cause the folder to excluded from `cargo package` so need to
        // be renamed
        if name.file_name() == Some(OsStr::new("_Cargo.toml")) {
            name.set_file_name("Cargo.toml");
        }

        let file_path = name.as_os_str().to_string_lossy();

        if path.is_file() {
            zip.start_file(file_path, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(file_path, options)?;
        }
    }
    zip.finish()?;

    Ok(())
}
