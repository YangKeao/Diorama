#![recursion_limit="128"]
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate quote;
#[macro_use]
extern crate parse_utils;

use syn::*;

#[derive(StringStructFromLit)]
struct UrlPath (pub String);
#[derive(StructFromAttrs)]
struct UrlParamsArgs {
    path: UrlPath,
}

#[proc_macro_derive(UrlParams)]
pub fn url_params(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let body = input.data;
    let attrs = input.attrs;

    let mut params = quote!{};
    match body {
        Data::Struct(data) => {
            let fields = data.fields.iter();
            for field in fields {
                let attrs = &field.attrs;
                let ident = field.ident.as_ref().unwrap();
                let ty = &field.ty;

                params = quote! (#params, #ident = self.#ident);
                println!("{}", params);
            }
        }
        _ => {
            panic!("Only Struct is supported for derive(UrlParams)")
        }
    }

    let UrlParamsArgs { path: UrlPath(path)} = &UrlParamsArgs::from_attrs(&attrs).unwrap();

    let output = quote! {
        impl #name {
            fn make_path(&self) -> String {
                format!(#path #params).unwrap()
            }
        }
    };
    println!("{}", output);
    output.into()
}