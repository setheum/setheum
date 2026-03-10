// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
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

//! Types common for current & legacy abft used across finality-setbft

use derive_more::{From, Into};
use parity_scale_codec::{Decode, Encode, Error, Input, Output};

/// The index of a node
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, From, Into)]
pub struct NodeIndex(pub usize);

impl Encode for NodeIndex {
    fn encode_to<T: Output + ?Sized>(&self, dest: &mut T) {
        (self.0 as u64).encode_to(dest);
    }
}

impl Decode for NodeIndex {
    fn decode<I: Input>(value: &mut I) -> Result<Self, Error> {
        Ok(NodeIndex(u64::decode(value)? as usize))
    }
}

/// Node count. Right now it doubles as node weight in many places in the code, in the future we
/// might need a new type for that.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, From, Into)]
pub struct NodeCount(pub usize);

/// A recipient of a message, either a specific node or everyone.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Recipient {
    Everyone,
    Node(NodeIndex),
}

impl From<set_bft::Recipient> for Recipient {
    fn from(recipient: set_bft::Recipient) -> Self {
        match recipient {
            set_bft::Recipient::Everyone => Recipient::Everyone,
            set_bft::Recipient::Node(id) => Recipient::Node(id.into()),
        }
    }
}

impl From<Recipient> for set_bft::Recipient {
    fn from(recipient: Recipient) -> Self {
        match recipient {
            Recipient::Everyone => set_bft::Recipient::Everyone,
            Recipient::Node(idx) => set_bft::Recipient::Node(idx.into()),
        }
    }
}


