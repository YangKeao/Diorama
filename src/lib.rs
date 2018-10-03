#![recursion_limit="128"]
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate quote;

use syn::{
    parse_macro_input,
    Ident,
    DeriveInput,
    Meta,
    Data,
    Type,
    punctuated::Punctuated,
    BareFnArg,
    token,
    Lit,
    LitStr,
    BareFnArgName,
    ReturnType};

#[proc_macro_derive(Diorama, attributes(base_url, method, path, parser))]
pub fn diorama(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let body = input.data;

    let mut base_url= Lit::from(LitStr::new("http:///", proc_macro2::Span::call_site())); // TODO: uninitialized => Compile Time Error
    for attr in input.attrs {
        let meta = attr.interpret_meta().unwrap();
        match meta {
            Meta::NameValue(meta_name_value) => {
                match meta_name_value.ident.to_string().as_ref() {
                    "base_url" => {
                        base_url = meta_name_value.lit;
                    }
                    _ => {
                        println!("ERROR");
                    }
                }
            }
            _ => {
                println!("ERROR");
            }
        }
    }

    let mut impl_body = quote! {};
    let mut field_body = quote! {};

    match body {
        Data::Struct(data) => {
            let fields = data.fields.iter();
            for field in fields {
                let attrs = &field.attrs;
                let ident = field.ident.as_ref().unwrap();
                let ty = &field.ty;

                let mut method: Lit = Lit::from(LitStr::new("GET", proc_macro2::Span::call_site()));
                let mut path: Lit = Lit::from(LitStr::new("/", proc_macro2::Span::call_site()));
                let mut parser: Lit = Lit::from(LitStr::new("JSON", proc_macro2::Span::call_site()));

                for attr in attrs {
                    let meta = attr.interpret_meta().unwrap();
                    match meta {
                        Meta::NameValue(meta_name_value) => {
                            match meta_name_value.ident.to_string().as_ref() {
                                "method" => {
                                    method = meta_name_value.lit;
                                }
                                "path" => {
                                    path = meta_name_value.lit;
                                }
                                "parser" => {
                                    parser = meta_name_value.lit;
                                }
                                _ => {
                                    println!("ERROR");
                                }
                            }
                        }
                        _ => {
                            println!("ERROR");
                        }
                    }
                }

                match ty {
                    Type::BareFn(fun) => {
                        let new_impl = impl_request(
                            &base_url,
                            &ident,
                            &fun.inputs,
                            &fun.output,
                            &path,
                            &method,
                            &parser
                        );
                        impl_body = quote! {
                            #impl_body
                            #new_impl
                        };
                        let new_field = field_request(
                            &base_url,
                            &ident,
                            &fun.inputs,
                            &fun.output,
                            &path,
                            &method,
                            &parser
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

fn impl_request(base_url: &Lit,
                ident: &Ident,
                inputs: &Punctuated<BareFnArg, token::Comma>,
                returns: &ReturnType,
                path: &Lit,
                method: &Lit,
                parser: &Lit) -> proc_macro2::TokenStream {
    let method = match method {
        Lit::Str(str) => {
            syn::Ident::new(&str.value()[..], proc_macro2::Span::call_site())
        }
        _ => {
            panic!("Error")
        }
    };
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

fn field_request(base_url: &Lit,
                ident: &Ident,
                inputs: &Punctuated<BareFnArg, token::Comma>,
                returns: &ReturnType,
                path: &Lit,
                method: &Lit,
                parser: &Lit) -> proc_macro2::TokenStream {
    quote! {
        #ident : |#inputs| #returns {
        },
    }
}