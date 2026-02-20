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

// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// Copyright (C) Use Ink (UK) Ltd.
// This file is part of cargo-contract.
//
// cargo-contract is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// cargo-contract is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with cargo-contract.  If not, see <http://www.gnu.org/licenses/>.

//! This file simply contains the end points of the production chains
//! We hard-code these values to ensure that a user uploads a verifiable bundle

use contract_extrinsics::url_to_string;
use std::str::FromStr;
use url::Url;

/// This macro generates enums with the pre-defined production chains and their respective
/// endpoints.
///
/// It also generates the required trait implementations.
macro_rules! define_chains {
    (
        $(#[$($attrs:tt)*])*
        pub enum $root:ident { $( $c:ident = ($ep:tt, $config:tt) ),* $(,)? }
    ) => {
        $(#[$($attrs)*])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum $root { $($c),* }

        impl $root {
            /// Returns the endpoint URL of a chain.
            pub fn url(&self) -> url::Url {
                match self {
                    $(
                        $root::$c => Url::parse($ep).expect("Incorrect Url format")
                    ),*
                }
            }

            /// Returns the config of a chain.
            pub fn config(&self) -> &str {
                match self {
                    $(
                        $root::$c => $config
                    ),*
                }
            }

            /// Returns the production chain.
            ///
            /// If the user specified the endpoint URL and config manually we'll attempt to
            /// convert it into one of the pre-defined production chains.
            pub fn from_parts(ep: &Url, config: &str) -> Option<Self> {
                match (url_to_string(ep).as_str(), config) {
                    $(
                        ($ep, $config) => Some($root::$c),
                    )*
                    _ => None
                }
            }
        }

       impl std::fmt::Display for $root {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                    $root::$c => f.write_str(stringify!($c))
                    ),*
                }
            }
        }

        impl FromStr for $root {
            type Err = anyhow::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        stringify!($c) => Ok($root::$c),
                    )*
                    _ => Err(anyhow::anyhow!("Unrecognised chain name"))
                }
            }
        }
    };
}

define_chains! {
    /// List of production chains where the contract can be deployed to.
    #[derive(clap::ValueEnum)]
    pub enum ProductionChain {
        AlephZero = ("wss://ws.azero.dev:443/", "Substrate"),
        Astar = ("wss://rpc.astar.network:443/", "Polkadot"),
        Shiden = ("wss://rpc.shiden.astar.network:443/", "Polkadot"),
        Krest = ("wss://wss-krest.peaq.network:443/", "Polkadot")
    }
}
