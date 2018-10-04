extern crate syn;
mod request;

pub fn impl_struct(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident;
    let body = input.data;
    let attrs = input.attrs;

    let client_meta = request::ClientMeta::from_attrs(&attrs).unwrap();

    let mut impl_body = quote! {};
    let mut field_body = quote! {};

    match body {
        syn::Data::Struct(data) => {
            let fields = data.fields.iter();
            for field in fields {
                let attrs = &field.attrs;
                let ident = field.ident.as_ref().unwrap();
                let ty = &field.ty;

                let req_meta = request::RequestMeta::from_attrs(attrs).unwrap();

                match ty {
                    syn::Type::BareFn(fun) => {
                        let new_impl = request::impl_request(
                            &client_meta,
                            &req_meta,
                            &ident,
                            &fun.output,
                        );
                        impl_body = quote! {
                            #impl_body
                            #new_impl
                        };
                        let new_field = request::impl_field_request(
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

    quote! {
        extern crate reqwest;
        impl #name {
            fn new() -> #name {
                #name {
                    #field_body
                }
            }
            #impl_body
        }
    }
}