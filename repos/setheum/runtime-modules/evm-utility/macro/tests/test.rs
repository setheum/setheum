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

#[cfg(test)]
mod tests {
	#[test]
	fn generate_function_selector_works() {
		#[module_evm_utility_macro::generate_function_selector]
		#[derive(Debug, Eq, PartialEq)]
		#[repr(u32)]
		pub enum Action {
			Name = "name()",
			Symbol = "symbol()",
			Decimals = "decimals()",
			TotalSupply = "totalSupply()",
			BalanceOf = "balanceOf(address)",
			Transfer = "transfer(address,uint256)",
		}

		assert_eq!(Action::Name as u32, 0x06fdde03_u32);
		assert_eq!(Action::Symbol as u32, 0x95d89b41_u32);
		assert_eq!(Action::Decimals as u32, 0x313ce567_u32);
		assert_eq!(Action::TotalSupply as u32, 0x18160ddd_u32);
		assert_eq!(Action::BalanceOf as u32, 0x70a08231_u32);
		assert_eq!(Action::Transfer as u32, 0xa9059cbb_u32);
	}

	#[test]
	fn keccak256_works() {
		assert_eq!(
			module_evm_utility_macro::keccak256!(""),
			&module_evm_utility::sha3_256("")
		);
		assert_eq!(
			module_evm_utility_macro::keccak256!("keccak256"),
			&module_evm_utility::sha3_256("keccak256")
		);
	}
}
