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

use ink_env::{
    DefaultEnvironment,
    Environment,
};
use std::{
    fmt::Debug,
    str::FromStr,
};
use subxt::{
    config::{
        PolkadotExtrinsicParams,
        SubstrateExtrinsicParams,
    },
    ext::{
        sp_core,
        sp_core::Pair,
    },
    tx::{
        PairSigner,
        Signer as SignerT,
    },
    Config,
    PolkadotConfig,
    SubstrateConfig,
};

/// Configuration for signer
pub trait SignerConfig<C: Config + Environment> {
    type Signer: SignerT<C> + FromStr + Clone;
}

/// A runtime configuration for the ecdsa test chain.
/// This thing is not meant to be instantiated; it is just a collection of types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ecdsachain {}

impl Config for Ecdsachain {
    type Hash = <SubstrateConfig as Config>::Hash;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = <SubstrateConfig as Config>::Address;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
    type AssetId = <SubstrateConfig as Config>::AssetId;
}

impl Environment for Ecdsachain {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;
    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type ChainExtension = <DefaultEnvironment as Environment>::ChainExtension;
}

impl SignerConfig<Self> for Ecdsachain
where
    <Self as Config>::Signature: From<sp_core::ecdsa::Signature>,
{
    type Signer = SignerEcdsa<Self>;
}

/// A runtime configuration for the Substrate based chain.
/// This thing is not meant to be instantiated; it is just a collection of types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Substrate {}

impl Config for Substrate {
    type Hash = <SubstrateConfig as Config>::Hash;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = <SubstrateConfig as Config>::Address;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
    type AssetId = <SubstrateConfig as Config>::AssetId;
}

impl Environment for Substrate {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;
    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type ChainExtension = <DefaultEnvironment as Environment>::ChainExtension;
}

impl SignerConfig<Self> for Substrate {
    type Signer = SignerSR25519<Self>;
}

/// A runtime configuration for the Polkadot based chain.
/// This thing is not meant to be instantiated; it is just a collection of types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Polkadot {}

impl Config for Polkadot {
    type Hash = <PolkadotConfig as Config>::Hash;
    type AccountId = <PolkadotConfig as Config>::AccountId;
    type Address = <PolkadotConfig as Config>::Address;
    type Signature = <PolkadotConfig as Config>::Signature;
    type Hasher = <PolkadotConfig as Config>::Hasher;
    type Header = <PolkadotConfig as Config>::Header;
    type ExtrinsicParams = PolkadotExtrinsicParams<Self>;
    type AssetId = <PolkadotConfig as Config>::AssetId;
}

impl Environment for Polkadot {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;
    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type ChainExtension = <DefaultEnvironment as Environment>::ChainExtension;
}

impl SignerConfig<Self> for Polkadot {
    type Signer = SignerSR25519<Self>;
}

/// Struct representing the implementation of the sr25519 signer
#[derive(Clone)]
pub struct SignerSR25519<C: Config>(pub PairSigner<C, sp_core::sr25519::Pair>);

impl<C: Config> FromStr for SignerSR25519<C>
where
    <C as Config>::AccountId: From<sp_core::crypto::AccountId32>,
{
    type Err = anyhow::Error;

    /// Attempts to parse the Signer suri string
    fn from_str(input: &str) -> Result<SignerSR25519<C>, Self::Err> {
        let keypair = sp_core::sr25519::Pair::from_string(input, None)?;
        let signer = PairSigner::<C, _>::new(keypair);
        Ok(Self(signer))
    }
}

impl<C: Config> SignerT<C> for SignerSR25519<C>
where
    <C as Config>::Signature: From<sp_core::sr25519::Signature>,
{
    fn account_id(&self) -> <C as Config>::AccountId {
        self.0.account_id().clone()
    }

    fn address(&self) -> C::Address {
        self.0.address()
    }

    fn sign(&self, signer_payload: &[u8]) -> C::Signature {
        self.0.sign(signer_payload)
    }
}

/// Struct representing the implementation of the ecdsa signer
#[derive(Clone)]
pub struct SignerEcdsa<C: Config>(pub PairSigner<C, sp_core::ecdsa::Pair>);

impl<C: Config> FromStr for SignerEcdsa<C>
where
    // Requirements of the `PairSigner where:
    // T::AccountId: From<SpAccountId32>`
    <C as Config>::AccountId: From<sp_core::crypto::AccountId32>,
{
    type Err = anyhow::Error;

    /// Attempts to parse the Signer suri string
    fn from_str(input: &str) -> Result<SignerEcdsa<C>, Self::Err> {
        let keypair = sp_core::ecdsa::Pair::from_string(input, None)?;
        let signer = PairSigner::<C, _>::new(keypair);
        Ok(Self(signer))
    }
}

impl<C: Config> SignerT<C> for SignerEcdsa<C>
where
    <C as Config>::Signature: From<sp_core::ecdsa::Signature>,
{
    fn account_id(&self) -> <C as Config>::AccountId {
        self.0.account_id().clone()
    }

    fn address(&self) -> C::Address {
        self.0.address()
    }

    fn sign(&self, signer_payload: &[u8]) -> C::Signature {
        self.0.sign(signer_payload)
    }
}

#[macro_export]
macro_rules! call_with_config_internal {
    ($obj:tt ,$function:tt, $config_name:expr, $( ($config_str:literal, $config_obj:ty) ),*) => {
        match $config_name {
            $(
                $config_str => $obj.$function::<$config_obj>().await,
            )*
            _ => {
              let configs = vec![$($config_str),*].iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ");
                Err(ErrorVariant::Generic(
                    contract_extrinsics::GenericError::from_message(
                        format!("Chain configuration {} not found, allowed configurations: {configs}", $config_name)
                )))
            },
        }
    };
}

/// Macro that allows calling the command member function with chain configuration
///
/// # Developer Note
///
/// In older Rust versions the macro `stringify!($crate::foo)` expanded to
/// `"$crate::foo"`. This behavior changed with https://github.com/rust-lang/rust/pull/125174,
/// `stringify!` expands to `"$crate :: foo"` now. In order to support both older and
/// newer Rust versions our macro has to handle both cases, spaced and non-spaced.
///
/// # Known Limitation
///
///  The `$config_name:expr` has to be in the `$crate::cmd::config` crate and cannot
/// contain  another `::` sub-path.
#[macro_export]
macro_rules! call_with_config {
    ($obj:tt, $function:ident, $config_name:expr) => {{
        assert!(
            !format!("{}", $config_name).contains("::"),
            "The supplied config name `{}` is not allowed to contain `::`.",
            $config_name
        );

        $crate::call_with_config_internal!(
            $obj,
            $function,
            $config_name,
            // All available chain configs need to be specified here
            ("Polkadot", $crate::cmd::config::Polkadot),
            ("Substrate", $crate::cmd::config::Substrate),
            ("Ecdsachain", $crate::cmd::config::Ecdsachain)
        )
    }};
}
