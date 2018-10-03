extern crate syn;
extern crate proc_macro2;

use syn::*;

fn lit_to_str(lit: &Lit) -> Option<String> {
    match lit {
        Lit::Str(s) => Some(s.value()),
        _ => None
    }
}

pub struct BaseUrl (pub String);
impl BaseUrl {
    fn from_lit(lit: &Lit) -> Result<Self, &str> {
        match lit_to_str(lit) {
            Some(str) => Ok(BaseUrl(str)),
            None => Err("base_url is not a String")
        }
    }
}

pub struct ClientMeta {
    pub base_url: BaseUrl
}
impl ClientMeta {
    pub fn from_attrs(attrs: &Vec<Attribute>) -> Result<Self, &str> {
        let mut base_url = BaseUrl("".to_owned());
        for attr in attrs {
            let meta = attr.interpret_meta().unwrap();
            match meta {
                Meta::NameValue(meta_name_value) => {
                    match meta_name_value.ident.to_string().as_ref() {
                        "base_url" => {
                            base_url = BaseUrl::from_lit(&meta_name_value.lit).unwrap();
                        }
                        _ => {
                            return Err("Unknown Attribute for client")
                        }
                    }
                }
                _ => {
                    return Err("Only NameValue Attribute is supported")
                }
            }
        }
        Ok(ClientMeta {
            base_url
        })
    }
}

pub struct UrlPath (pub String);
impl UrlPath {
    fn from_lit(lit: &Lit) -> Result<Self, &str> {
        match lit_to_str(lit) {
            Some(str) => Ok(UrlPath(str)),
            None => Err("Path is not a String")
        }
    }
}

pub enum Method {
    GET,
    POST,
    DELETE
}
impl Method {
    fn from_lit(lit: &Lit) -> Result<Self, &str> {
        match &lit_to_str(lit).unwrap()[..] {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "DELETE" => Ok(Method::DELETE),
            _ => Err("Unknown Method")
        }
    }
    pub fn to_ident(&self) -> Ident {
        match self {
            Method::GET => Ident::new("get", proc_macro2::Span::call_site()),
            Method::POST => Ident::new("post", proc_macro2::Span::call_site()),
            Method::DELETE => Ident::new("delete", proc_macro2::Span::call_site()),
        }
    }
}

pub enum Parser {
    Html,
    Json
}
impl Parser {
    fn from_lit(lit: &Lit) -> Result<Self, &str> {
        match &lit_to_str(lit).unwrap()[..] {
            "html" => Ok(Parser::Html),
            "json" => Ok(Parser::Json),
            _ => Err("Unknown Parser")
        }
    }
}

pub struct RequestMeta {
    pub method: Method,
    pub path: UrlPath,
    pub parser: Parser
}
impl RequestMeta {
    pub fn from_attrs(attrs: &Vec<Attribute>) -> Result<Self, &str> {
        let mut method: Method = Method::GET;
        let mut path: UrlPath = UrlPath("/".to_owned());
        let mut parser: Parser = Parser::Json;
        for attr in attrs {
            let meta = attr.interpret_meta().unwrap();
            match meta {
                Meta::NameValue(meta_name_value) => {
                    match meta_name_value.ident.to_string().as_ref() {
                        "method" => {
                            method = Method::from_lit(&meta_name_value.lit).unwrap();
                        }
                        "path" => {
                            path = UrlPath::from_lit(&meta_name_value.lit).unwrap();
                        }
                        "parser" => {
                            parser = Parser::from_lit(&meta_name_value.lit).unwrap();
                        }
                        _ => {
                            return Err("Unknown Meta Message")
                        }
                    }
                }
                _ => {
                    return Err("Only NameValue Attribute is supported")
                }
            }
        }
        Ok(RequestMeta {
            method,
            path,
            parser
        })
    }
}