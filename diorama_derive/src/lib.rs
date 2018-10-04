#![recursion_limit="128"]
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

extern crate diorama_core;

use syn::*;

#[proc_macro_derive(Diorama, attributes(base_url, method, path, parser))]
pub fn diorama(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let output = diorama_core::impl_struct(input);
    output.into()
}

#[proc_macro_derive(UrlParams, attributes(path))]
pub fn url_params(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let output = diorama_core::impl_make_path(input);
    output.into()
}