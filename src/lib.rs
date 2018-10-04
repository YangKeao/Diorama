#![recursion_limit="128"]
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate parse_utils;
#[macro_use]
extern crate url_params;
#[macro_use]
extern crate quote;

mod request;
use self::request::*;

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
        _ => {
            panic!("Only Struct is supported for derive(Diorama)")
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
                returns: &ReturnType) -> proc_macro2::TokenStream {
    let method = req_meta.method.to_ident();
    let BaseUrl( base_url) = &client_meta.base_url;

    quote! {
        fn #ident (&self, url_params: T, body: &'static str) -> Result<String, reqwest::Error> {
            let url = &format!("{}{}", #base_url, url_params.make_path())[..];
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
                 returns: &ReturnType) -> proc_macro2::TokenStream {
    quote! {
        #ident : || #returns {
        },
    }
}