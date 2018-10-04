extern crate syn;

#[derive(StringStructFromLit)]
struct UrlPath (pub String);
#[derive(StructFromAttrs)]
struct UrlParamsArgs {
    path: UrlPath,
}

pub fn impl_make_path(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident;
    let body = input.data;
    let attrs = input.attrs;

    let mut params = quote!{};
    match body {
        syn::Data::Struct(data) => {
            let fields = data.fields.iter();
            for field in fields {
                let attrs = &field.attrs;
                let ident = field.ident.as_ref().unwrap();
                let ty = &field.ty;

                params = quote! (#params, #ident = self.#ident);
            }
        }
        _ => {
            panic!("Only Struct is supported for derive(UrlParams)")
        }
    }

    let UrlParamsArgs { path: UrlPath(path)} = &UrlParamsArgs::from_attrs(&attrs).unwrap();

    let output = quote! {
        impl diorama::UrlParams for #name {
            fn make_path(&self) -> String {
                format!(#path #params)
            }
        }
    };
    println!("{}", output);
    output.into()
}