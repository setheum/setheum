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

//! This module declares an `Executor` which is either a
//! * `WasmExecutor`, for production and test build (when no local debugging is required)
//! * `NativeElseWasmExecutor` for `runtime-benchmarks` and local debugging builds

use sc_service::Configuration;

#[cfg(not(any(feature = "runtime-benchmarks", feature = "setheum-native-runtime",)))]
pub mod executor {
	use sc_executor::WasmExecutor;

	use super::Configuration;

	type ExtendHostFunctions = (sp_io::SubstrateHostFunctions,);
	pub type Executor = WasmExecutor<ExtendHostFunctions>;

	pub fn get_executor(config: &Configuration) -> Executor {
		sc_service::new_wasm_executor(config)
	}
}

#[cfg(any(feature = "runtime-benchmarks", feature = "setheum-native-runtime",))]
pub mod executor {
	use sc_executor::NativeElseWasmExecutor;

	use super::Configuration;

	pub struct ExecutorDispatch;

	impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
		#[cfg(feature = "runtime-benchmarks")]
		type ExtendHostFunctions = (frame_benchmarking::benchmarking::HostFunctions,);

		#[cfg(not(feature = "runtime-benchmarks"))]
		type ExtendHostFunctions = ();

		fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
			setheum_runtime::api::dispatch(method, data)
		}

		fn native_version() -> sc_executor::NativeVersion {
			setheum_runtime::native_version()
		}
	}

	pub type Executor = NativeElseWasmExecutor<ExecutorDispatch>;

	pub fn get_executor(config: &Configuration) -> Executor {
		sc_service::new_native_or_wasm_executor(config)
	}
}
