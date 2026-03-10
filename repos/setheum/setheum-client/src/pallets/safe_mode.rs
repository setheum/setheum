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

use crate::{
    setheum_runtime::RuntimeCall::SafeMode,
    api,
    connections::TxInfo,
    pallet_safe_mode::pallet::Call::{force_enter, force_exit, force_extend},
    AsConnection, BlockHash, BlockNumber, ConnectionApi, RootConnection, SignedConnectionApi,
    SudoCall, TxStatus,
};

/// Pallet SafeMode API which does not require sudo.
#[async_trait::async_trait]
pub trait SafeModeApi {
    /// Gets the last block number that the safe-mode will remain entered in
    async fn entered_until(&self, at: Option<BlockHash>) -> Option<BlockNumber>;

    /// Gets the safe mode enter and extend duration.
    async fn safe_mode_config(&self) -> (BlockNumber, BlockNumber);
}

/// Pallet SafeMode user API.
#[async_trait::async_trait]
pub trait SafeModeUserApi {
    /// API for [`enter`](https://paritytech.github.io/polkadot-sdk/master/pallet_safe_mode/pallet/struct.Pallet.html#method.enter) call.
    async fn enter(&self, status: TxStatus) -> anyhow::Result<TxInfo>;

    /// API for [`extend`](https://paritytech.github.io/polkadot-sdk/master/pallet_safe_mode/pallet/struct.Pallet.html#method.extend) call.
    async fn extend(&self, status: TxStatus) -> anyhow::Result<TxInfo>;
}

/// Pallet SafeMode API that requires sudo.
#[async_trait::async_trait]
pub trait SafeModeSudoApi {
    /// API for [`force_enter`](https://paritytech.github.io/polkadot-sdk/master/pallet_safe_mode/pallet/struct.Pallet.html#method.force_enter) call.
    async fn force_enter(&self, status: TxStatus) -> anyhow::Result<TxInfo>;

    /// API for [`force_extend`](https://paritytech.github.io/polkadot-sdk/master/pallet_safe_mode/pallet/struct.Pallet.html#method.force_extend) call.
    async fn force_extend(&self, status: TxStatus) -> anyhow::Result<TxInfo>;

    /// API for [`force_exit`](https://paritytech.github.io/polkadot-sdk/master/pallet_safe_mode/pallet/struct.Pallet.html#method.force_exit) call.
    async fn force_exit(&self, status: TxStatus) -> anyhow::Result<TxInfo>;
}

#[async_trait::async_trait]
impl<C: ConnectionApi + AsConnection> SafeModeApi for C {
    async fn entered_until(&self, at: Option<BlockHash>) -> Option<BlockNumber> {
        let addrs = api::storage().safe_mode().entered_until();

        self.get_storage_entry_maybe(&addrs, at).await
    }

    async fn safe_mode_config(&self) -> (BlockNumber, BlockNumber) {
        let enter_duration_addrs = api::constants().safe_mode().enter_duration();
        let extend_duration_addrs = api::constants().safe_mode().extend_duration();

        let enter_duration = self
            .as_connection()
            .as_client()
            .constants()
            .at(&enter_duration_addrs)
            .expect("Constant should be set on chain");
        let extend_duration = self
            .as_connection()
            .as_client()
            .constants()
            .at(&extend_duration_addrs)
            .expect("Constant should be set on chain");

        (enter_duration, extend_duration)
    }
}

#[async_trait::async_trait]
impl<S: SignedConnectionApi> SafeModeUserApi for S {
    async fn enter(&self, status: TxStatus) -> anyhow::Result<TxInfo> {
        let tx = api::tx().safe_mode().enter();

        self.send_tx(tx, status).await
    }

    async fn extend(&self, status: TxStatus) -> anyhow::Result<TxInfo> {
        let tx = api::tx().safe_mode().extend();

        self.send_tx(tx, status).await
    }
}

#[async_trait::async_trait]
impl SafeModeSudoApi for RootConnection {
    async fn force_enter(&self, status: TxStatus) -> anyhow::Result<TxInfo> {
        let call = SafeMode(force_enter);

        self.sudo_unchecked(call, status).await
    }

    async fn force_extend(&self, status: TxStatus) -> anyhow::Result<TxInfo> {
        let call = SafeMode(force_extend);

        self.sudo_unchecked(call, status).await
    }

    async fn force_exit(&self, status: TxStatus) -> anyhow::Result<TxInfo> {
        let call = SafeMode(force_exit);

        self.sudo_unchecked(call, status).await
    }
}
