#![recursion_limit="128"]
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

mod request;
use self::request::*;
mod utils;
use self::utils::*;

#[macro_use]
extern crate quote;

use syn::*;

#[proc_macro_derive(Diorama, attributes(base_url, method, path, parser))]
pub fn diorama(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let body = input.data;
    let attrs = input.attrs;

    let client_meta = ClientMeta::from_attrs(&attrs).unwrap();

    let mut impl_body = quote! {};
    let mut field_body = quote! {};

    match body {
        Data::Struct(data) => {
            let fields = data.fields.iter();
            for field in fields {
                let attrs = &field.attrs;
                let ident = field.ident.as_ref().unwrap();
                let ty = &field.ty;

                let req_meta = RequestMeta::from_attrs(attrs).unwrap();

                match ty {
                    Type::BareFn(fun) => {
                        let new_impl = impl_request(
                            &client_meta,
                            &req_meta,
                            &ident,
                            &fun.inputs,
                            &fun.output,
                        );
                        impl_body = quote! {
                            #impl_body
                            #new_impl
                        };
                        let new_field = field_request(
                            &client_meta,
                            &req_meta,
                            &ident,
                            &fun.inputs,
                            &fun.output,
                        );
                        field_body = quote! {
                            #field_body
                            #new_field
                        }
                    }
                    _ => {
                        println!("ERROR");
                    }
                }
            }
        }
        Data::Enum(_data) => {

        }
        Data::Union(_data) => {

        }
    }

    let output: proc_macro2::TokenStream = quote! {
        extern crate reqwest;
        impl #name {
            fn new() -> #name {
                #name {
                    #field_body
                }
            }
            #impl_body
        }
    };

    output.into()
}

fn impl_request(client_meta: &ClientMeta,
                req_meta: &RequestMeta,
                ident: &Ident,
                inputs: &punctuated::Punctuated<BareFnArg, token::Comma>,
                returns: &ReturnType) -> proc_macro2::TokenStream {
    let method = req_meta.method.to_ident();
    let UrlPath( path ) = &req_meta.path;
    let BaseUrl( base_url) = &client_meta.base_url;

    let mut url_params = quote! ();
    for arg in inputs {
        let name = &arg.name.as_ref().unwrap().0;
        match name {
            BareFnArgName::Named(ident) => {
                url_params = quote! (#url_params, #ident = #ident);
            }
            _ => {
                println!("ERROR")
            }
        }
    }
    quote! {
        fn #ident (&self, #inputs, body: &'static str) -> Result<String, reqwest::Error> {
            let url = &format!("{}{}", #base_url, format!(#path #url_params))[..];
            let client = reqwest::Client::new();

            let res = client.#method(url)
                .body(body)
                .send()?
                .text()?;

            println!("{}", res);

            return Ok(res);
        }
    }
}

fn field_request(_: &ClientMeta,
                 _: &RequestMeta,
                 ident: &Ident,
                 inputs: &punctuated::Punctuated<BareFnArg, token::Comma>,
                 returns: &ReturnType) -> proc_macro2::TokenStream {
    quote! {
        #ident : |#inputs| #returns {
        },
    }
}