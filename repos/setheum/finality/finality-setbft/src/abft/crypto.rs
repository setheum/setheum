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

use crate::{
    crypto::{AuthorityPen, AuthorityVerifier, Signature},
    NodeCount, NodeIndex, SignatureSet,
};

/// Keychain combines an AuthorityPen and AuthorityVerifier into one object implementing the SetBFT
/// MultiKeychain trait.
#[derive(Clone)]
pub struct Keychain {
    id: NodeIndex,
    authority_pen: AuthorityPen,
    authority_verifier: AuthorityVerifier,
}

impl Keychain {
    /// Constructs a new keychain from a signing contraption and verifier, with the specified node
    /// index.
    pub fn new(
        id: NodeIndex,
        authority_verifier: AuthorityVerifier,
        authority_pen: AuthorityPen,
    ) -> Self {
        Keychain {
            id,
            authority_pen,
            authority_verifier,
        }
    }

    fn index(&self) -> NodeIndex {
        self.id
    }

    fn node_count(&self) -> NodeCount {
        self.authority_verifier.node_count()
    }

    fn sign(&self, msg: &[u8]) -> Signature {
        self.authority_pen.sign(msg)
    }

    fn verify<I: Into<NodeIndex>>(&self, msg: &[u8], sgn: &Signature, index: I) -> bool {
        self.authority_verifier.verify(msg, sgn, index.into())
    }

    fn is_complete(&self, msg: &[u8], partial: &SignatureSet<Signature>) -> bool {
        self.authority_verifier.is_complete(msg, partial)
    }
}

// Currently the traits for legacy and current match, so only one implementation needed.
impl set_bft::Index for Keychain {
    fn index(&self) -> set_bft::NodeIndex {
        Keychain::index(self).into()
    }
}

// Currently the traits for legacy and current match, so only one implementation needed.
impl set_bft::Keychain for Keychain {
    type Signature = Signature;

    fn node_count(&self) -> set_bft::NodeCount {
        Keychain::node_count(self).into()
    }

    fn sign(&self, msg: &[u8]) -> Signature {
        Keychain::sign(self, msg)
    }

    fn verify(&self, msg: &[u8], sgn: &Signature, index: set_bft::NodeIndex) -> bool {
        Keychain::verify(self, msg, sgn, index)
    }
}

// Currently the traits for legacy and current match, so only one implementation needed.
impl set_bft::MultiKeychain for Keychain {
    // Using `SignatureSet` is slow, but Substrate has not yet implemented aggregation.
    // We probably should do this for them at some point.
    type PartialMultisignature = SignatureSet<Signature>;

    fn bootstrap_multi(
        &self,
        signature: &Signature,
        index: set_bft::NodeIndex,
    ) -> Self::PartialMultisignature {
        set_bft::PartialMultisignature::add_signature(
            SignatureSet(set_bft_crypto::SignatureSet::with_size(
                set_bft_crypto::Keychain::node_count(self),
            )),
            signature,
            index,
        )
    }

    fn is_complete(&self, msg: &[u8], partial: &Self::PartialMultisignature) -> bool {
        Keychain::is_complete(self, msg, partial)
    }
}
