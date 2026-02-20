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

use crate::{
    engine::off_chain::{
        impls::TopicsBuilder,
        test_api::set_account_balance,
    },
    event::TopicsBuilderBackend,
    types::Environment,
    DefaultEnvironment,
    Result,
};

#[test]
fn topics_builder() -> Result<()> {
    crate::test::run_test::<crate::DefaultEnvironment, _>(|_| {
        // given
        let mut builder = TopicsBuilder::default();

        // when
        TopicsBuilderBackend::<crate::DefaultEnvironment>::push_topic(&mut builder, &13);
        TopicsBuilderBackend::<crate::DefaultEnvironment>::push_topic(&mut builder, &17);

        // then
        assert_eq!(builder.topics.len(), 2);

        let topics_len_compact = &scale::Compact(2u32);
        let topics_len_encoded = scale::Encode::encode(&topics_len_compact);
        let output = TopicsBuilderBackend::<crate::DefaultEnvironment>::output(builder);
        #[rustfmt::skip]
        let expected = vec![topics_len_encoded[0], 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(output, expected);

        Ok(())
    })
}
#[test]
fn test_set_account_balance() -> Result<()> {
    pub use ink_engine::ext::ChainSpec;

    crate::test::run_test::<DefaultEnvironment, _>(|_| {
        let minimum_balance = ChainSpec::default().minimum_balance;

        let result = std::panic::catch_unwind(|| {
            set_account_balance::<DefaultEnvironment>(
                <DefaultEnvironment as Environment>::AccountId::from([0x1; 32]),
                <DefaultEnvironment as Environment>::Balance::from(minimum_balance - 1),
            )
        });

        assert!(result.is_err());

        set_account_balance::<DefaultEnvironment>(
            <DefaultEnvironment as Environment>::AccountId::from([0x1; 32]),
            <DefaultEnvironment as Environment>::Balance::from(0u128),
        );

        set_account_balance::<DefaultEnvironment>(
            <DefaultEnvironment as Environment>::AccountId::from([0x1; 32]),
            <DefaultEnvironment as Environment>::Balance::from(minimum_balance + 1),
        );

        Ok(())
    })
}
