#[derive(StringStructFromLit)]
pub struct BaseUrl (pub String);

#[derive(StructFromAttrs)]
pub struct ClientMeta {
    pub base_url: BaseUrl
}

#[derive(EnumFromLit)]
pub enum Method {
    GET,
    POST,
    DELETE
}

impl Method {
    pub fn to_ident(&self) -> syn::Ident {
        match self {
            Method::GET => syn::Ident::new("get", proc_macro2::Span::call_site()),
            Method::POST => syn::Ident::new("post", proc_macro2::Span::call_site()),
            Method::DELETE => syn::Ident::new("delete", proc_macro2::Span::call_site()),
        }
    }
}

#[derive(EnumFromLit)]
pub enum Parser {
    Html,
    Json
}

#[derive(StructFromAttrs)]
pub struct RequestMeta {
    pub method: Method,
    pub parser: Parser
}


pub fn impl_request(client_meta: &ClientMeta,
                req_meta: &RequestMeta,
                ident: &syn::Ident,
                returns: &syn::ReturnType) -> proc_macro2::TokenStream {
    let method = req_meta.method.to_ident();
    let BaseUrl( base_url) = &client_meta.base_url;

    quote! {
        fn #ident <T: diorama::UrlParams>(&self, url_params: T, body: &'static str) -> Result<String, reqwest::Error> {
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

pub fn impl_field_request(_: &ClientMeta,
                          _: &RequestMeta,
                          ident: &syn::Ident,
                          returns: &syn::ReturnType) -> proc_macro2::TokenStream {
    quote! {
        #ident : || #returns {
        },
    }
}