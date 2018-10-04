#![recursion_limit="128"]
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate quote;

#[proc_macro_derive(StringStructFromLit)]
pub fn string_struct_from_lit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;
    let name_str = &ident_to_litstr(&input.ident);

    let output = quote! {
        impl #name {
            fn from_lit(lit: &syn::Lit) -> Result<Self, String> {
                let str = match lit {
                    syn::Lit::Str(s) => Some(s.value()),
                    _ => None
                };
                match str {
                    Some(str) => Ok(#name(str)),
                    None => Err(format!("Parse {} error. It is not a String", #name_str))
                }
            }
            fn new() -> Self {
                #name("".to_string())
            }
        }
    };
    output.into()
}

#[proc_macro_derive(EnumFromLit)]
pub fn enum_from_lit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;
    let name_str = &ident_to_litstr(&input.ident);
    let body = input.data;

    let mut default_value= syn::Ident::new("Uninitialized", proc_macro2::Span::call_site());

    let mut match_body = quote!{};
    match body {
        syn::Data::Enum(data) => {
            let fields = data.variants;
            for field in fields {
                let field_ident = &field.ident;
                let field_name = &ident_to_litstr(&field.ident);
                match_body = quote! {
                    #match_body
                    #field_name => Ok(#name::#field_ident),
                };

                default_value = field.ident;
            }
        },
        _ => {
            panic!("EnumFromLit must be used on Enum!")
        }
    }

    let output = quote! {
        impl #name {
            fn from_lit(lit: &syn::Lit) -> Result<Self, String> {
                let str = match lit {
                    syn::Lit::Str(s) => Some(s.value()),
                    _ => None
                };
                match &str.unwrap()[..] {
                    #match_body
                    _ => Err(format!("Unknown {}", #name_str))
                }
            }
            fn new() -> Self {
                #name::#default_value
            }
        }
    };
    output.into()
}

#[proc_macro_derive(StructFromAttrs, attributes(default))]
pub fn struct_from_attrs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;
    let name_str = &ident_to_litstr(&input.ident);
    let body = &input.data;

    let mut initial_part = quote! {};
    let mut match_body = quote! {};
    let mut variant_body = quote! {};

    match body {
        syn::Data::Struct(data) => {
            let fields = data.fields.iter();
            for field in fields {
                let attrs = &field.attrs;
                let ident = field.ident.as_ref().unwrap();
                let ident_str = &ident_to_litstr(ident);
                let ty = &field.ty;

                initial_part = quote! {
                    #initial_part
                    let mut #ident: #ty = #ty::new();
                };

                match_body = quote! {
                    #match_body
                    #ident_str => {
                        #ident = #ty::from_lit(&meta_name_value.lit).unwrap();
                    }
                };

                variant_body = quote! {
                    #variant_body
                    #ident,
                }
            }
        },
        _ => {
            panic!("StructFromAttrs must be used on Struct!")
        }
    }

    let output = quote! {
        impl #name {
            pub fn from_attrs(attrs: &Vec<syn::Attribute>) -> Result<Self, String> {
                #initial_part
                for attr in attrs {
                    let meta = attr.interpret_meta().unwrap();
                    match meta {
                        syn::Meta::NameValue(meta_name_value) => {
                            match meta_name_value.ident.to_string().as_ref() {
                                #match_body
                                _ => {
                                    return Err(format!("Unknown Meta Message in {}", #name_str))
                                }
                            }
                        }
                        _ => {
                            return Err("Only NameValue Attribute is supported".to_string())
                        }
                    }
                }
                Ok(#name {
                    #variant_body
                })
            }
        }
    };
    output.into()
}

fn ident_to_litstr(ident: &syn::Ident) -> syn::LitStr {
    syn::LitStr::new(&ident.to_string()[..], proc_macro2::Span::call_site())
}