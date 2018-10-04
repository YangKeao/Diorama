use syn::*;

#[derive(StringStructFromLit)]
pub struct BaseUrl (pub String);

#[derive(StructFromAttrs)]
pub struct ClientMeta {
    pub base_url: BaseUrl
}

#[derive(StringStructFromLit)]
pub struct UrlPath (pub String);

#[derive(EnumFromLit)]
pub enum Method {
    GET,
    POST,
    DELETE
}

impl Method {
    pub fn to_ident(&self) -> Ident {
        match self {
            Method::GET => Ident::new("get", proc_macro2::Span::call_site()),
            Method::POST => Ident::new("post", proc_macro2::Span::call_site()),
            Method::DELETE => Ident::new("delete", proc_macro2::Span::call_site()),
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
    pub path: UrlPath,
    pub parser: Parser
}