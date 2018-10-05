pub trait UrlParams {
    fn make_path(&self) -> String ;
}

pub trait Deserializer {
    fn deserialize(content: &String) -> Self;
}

pub trait Serializer {
    fn serialize(&self) -> String;
}