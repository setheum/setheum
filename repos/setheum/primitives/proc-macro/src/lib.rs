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

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum, Expr, Lit};
use tiny_keccak::{Hasher, Keccak};

#[proc_macro_attribute]
pub fn generate_function_selector(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemEnum);

    for variant in &mut input.variants {
        if let Some((_, Expr::Lit(lit))) = &variant.discriminant {
            if let Lit::Str(lit_str) = &lit.lit {
                let func_sig = lit_str.value();
                let selector = keccak_256(func_sig.as_bytes());
                let selector_u32 = u32::from_be_bytes([selector[0], selector[1], selector[2], selector[3]]);
                
                let new_discriminant = syn::parse_str::<Expr>(&format!("0x{:08x}", selector_u32)).unwrap();
                variant.discriminant = Some((Default::default(), new_discriminant));
            }
        }
    }

    TokenStream::from(quote! {
        #input
    })
}

fn keccak_256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(input);
    hasher.finalize(&mut output);
    output
}
