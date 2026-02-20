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
