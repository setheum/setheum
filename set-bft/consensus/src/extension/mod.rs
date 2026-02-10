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

use crate::{dag::DagUnit, MultiKeychain, UnitFinalizationHandler};

mod election;
mod extender;
mod units;

use extender::Extender;

/// A struct responsible for executing the Consensus protocol on a local copy of the Dag.
/// It receives units which are guaranteed to eventually appear in the Dags
/// of all honest nodes. The static Aleph Consensus algorithm is then run on this Dag in order
/// to finalize subsequent rounds of the Dag. More specifically whenever a new unit is received
/// this process checks whether a new round can be finalized and if so, it computes the batch of
/// units that should be finalized, and uses the finalization handler to report that to the user.
///
/// We refer to the documentation https://cardinal-cryptography.github.io/AlephBFT/internals.html
/// Section 5.4 for a discussion of this component.
pub struct Ordering<MK: MultiKeychain, UFH: UnitFinalizationHandler> {
    extender: Extender<DagUnit<UFH::Hasher, UFH::Data, MK>>,
    finalization_handler: UFH,
}

impl<MK: MultiKeychain, UFH: UnitFinalizationHandler> Ordering<MK, UFH> {
    pub fn new(finalization_handler: UFH) -> Self {
        let extender = Extender::new();
        Ordering {
            extender,
            finalization_handler,
        }
    }

    pub fn add_unit(&mut self, unit: DagUnit<UFH::Hasher, UFH::Data, MK>) {
        for batch in self.extender.add_unit(unit) {
            self.finalization_handler
                .batch_finalized(batch.into_iter().map(|unit| unit.into()).collect());
        }
    }
}
