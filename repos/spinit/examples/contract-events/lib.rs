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

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod flipper {
    #[ink::event]
    pub struct Flipped {
        new_value: bool,
    }

    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    impl Flipper {
        #[ink(constructor)]
        pub fn new(init: bool) -> Self {
            Self { value: init }
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
            self.env().emit_event(Flipped {
                new_value: self.value,
            });
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use drink::session::{Session, NO_ARGS, NO_ENDOWMENT};

    #[drink::contract_bundle_provider]
    enum BundleProvider {}

    #[drink::test]
    fn we_can_inspect_emitted_events(mut session: Session) -> Result<(), Box<dyn Error>> {
        let bundle = BundleProvider::local()?;

        // Firstly, we deploy the contract and call its `flip` method.
        session.deploy_bundle(bundle.clone(), "new", &["false"], None, NO_ENDOWMENT)?;
        let _ = session.call::<_, ()>("flip", NO_ARGS, NO_ENDOWMENT)??;

        // Now we can inspect the emitted events.
        let record = session.record();
        let contract_events = record
            .last_event_batch()
            // We can use the `contract_events_decoded` method to decode the events into
            // `contract_transcode::Value` objects.
            .contract_events_decoded(&bundle.transcoder);

        assert_eq!(contract_events.len(), 1);
        println!("flip_event: {:?}", &contract_events[0]);

        Ok(())
    }
}
