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
use contract_build::Verbosity;
use derivative::Derivative;
use ink_env::Environment;
use subxt::{
    tx,
    Config,
};
use url::Url;

use crate::{
    url_to_string,
    ContractArtifacts,
};
use std::{
    marker::PhantomData,
    option::Option,
    path::PathBuf,
};

/// Arguments required for creating and sending an extrinsic to a Substrate node.
#[derive(Derivative)]
#[derivative(Clone(bound = "E::Balance: Clone"))]
pub struct ExtrinsicOpts<C: Config, E: Environment, Signer: Clone> {
    file: Option<PathBuf>,
    manifest_path: Option<PathBuf>,
    url: url::Url,
    signer: Signer,
    storage_deposit_limit: Option<E::Balance>,
    verbosity: Verbosity,
    _marker: PhantomData<C>,
}

/// A builder for extrinsic options.
pub struct ExtrinsicOptsBuilder<C: Config, E: Environment, Signer: Clone> {
    opts: ExtrinsicOpts<C, E, Signer>,
}

impl<C: Config, E: Environment, Signer> ExtrinsicOptsBuilder<C, E, Signer>
where
    Signer: tx::Signer<C> + Clone,
{
    /// Returns a clean builder for [`ExtrinsicOpts`].
    pub fn new(signer: Signer) -> ExtrinsicOptsBuilder<C, E, Signer> {
        ExtrinsicOptsBuilder {
            opts: ExtrinsicOpts {
                file: None,
                manifest_path: None,
                url: url::Url::parse("ws://localhost:9944").unwrap(),
                signer,
                storage_deposit_limit: None,
                verbosity: Verbosity::Default,
                _marker: PhantomData,
            },
        }
    }

    /// Sets the path to the contract build artifact file.
    pub fn file<T: Into<PathBuf>>(self, file: Option<T>) -> Self {
        let mut this = self;
        this.opts.file = file.map(|f| f.into());
        this
    }

    /// Sets the path to the Cargo.toml of the contract.
    pub fn manifest_path<T: Into<PathBuf>>(self, manifest_path: Option<T>) -> Self {
        let mut this = self;
        this.opts.manifest_path = manifest_path.map(|f| f.into());
        this
    }

    /// Sets the websockets url of a Substrate node.
    pub fn url<T: Into<Url>>(self, url: T) -> Self {
        let mut this = self;
        this.opts.url = url.into();
        this
    }

    /// Sets the maximum amount of balance that can be charged from the caller to pay for
    /// storage.
    pub fn storage_deposit_limit(
        self,
        storage_deposit_limit: Option<E::Balance>,
    ) -> Self {
        let mut this = self;
        this.opts.storage_deposit_limit = storage_deposit_limit;
        this
    }

    /// Set the verbosity level.
    pub fn verbosity(self, verbosity: Verbosity) -> Self {
        let mut this = self;
        this.opts.verbosity = verbosity;
        this
    }

    pub fn done(self) -> ExtrinsicOpts<C, E, Signer> {
        self.opts
    }
}

impl<C: Config, E: Environment, Signer> ExtrinsicOpts<C, E, Signer>
where
    Signer: tx::Signer<C> + Clone,
{
    /// Load contract artifacts.
    pub fn contract_artifacts(&self) -> Result<ContractArtifacts> {
        ContractArtifacts::from_manifest_or_file(
            self.manifest_path.as_ref(),
            self.file.as_ref(),
        )
    }

    /// Return the file path of the contract artifact.
    pub fn file(&self) -> Option<&PathBuf> {
        self.file.as_ref()
    }

    /// Return the path to the `Cargo.toml` of the contract.
    pub fn manifest_path(&self) -> Option<&PathBuf> {
        self.manifest_path.as_ref()
    }

    /// Return the URL of the Substrate node.
    pub fn url(&self) -> String {
        url_to_string(&self.url)
    }

    /// Return the signer.
    pub fn signer(&self) -> &Signer {
        &self.signer
    }

    /// Return the storage deposit limit.
    pub fn storage_deposit_limit(&self) -> Option<E::Balance> {
        self.storage_deposit_limit
    }

    /// Verbosity for message reporting.
    pub fn verbosity(&self) -> &Verbosity {
        &self.verbosity
    }
}
