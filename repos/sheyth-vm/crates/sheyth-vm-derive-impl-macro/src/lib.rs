#![allow(warnings)]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

extern crate proc_macro;

use proc_macro::TokenStream;

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn __PRIVATE_DO_NOT_USE_sheyth_vm_import(args: TokenStream, input: TokenStream) -> TokenStream {
    let attributes = syn::parse_macro_input!(args as sheyth_vm_derive_impl::ImportBlockAttributes);
    let input = syn::parse_macro_input!(input as syn::ItemForeignMod);
    match sheyth_vm_derive_impl::sheyth_vm_import(attributes, input) {
        Ok(result) => result.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn __PRIVATE_DO_NOT_USE_sheyth_vm_export(args: TokenStream, input: TokenStream) -> TokenStream {
    let attributes = syn::parse_macro_input!(args as sheyth_vm_derive_impl::ExportBlockAttributes);
    let input = syn::parse_macro_input!(input as syn::ItemFn);
    match sheyth_vm_derive_impl::sheyth_vm_export(attributes, input) {
        Ok(result) => result.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn __PRIVATE_DO_NOT_USE_sheyth_vm_define_abi(args: TokenStream, input: TokenStream) -> TokenStream {
    let attributes = syn::parse_macro_input!(args as sheyth_vm_derive_impl::AbiSupportAttributes);
    let input = syn::parse_macro_input!(input as syn::ItemMod);
    match sheyth_vm_derive_impl::sheyth_vm_define_abi(attributes, input) {
        Ok(result) => result.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

#[allow(non_snake_case)]
#[proc_macro]
pub fn __PRIVATE_DO_NOT_USE_sheyth_vm_impl_abi_support(input: TokenStream) -> TokenStream {
    let attributes = syn::parse_macro_input!(input as sheyth_vm_derive_impl::AbiSupportAttributes);
    sheyth_vm_derive_impl::sheyth_vm_impl_abi_support(attributes).into()
}
