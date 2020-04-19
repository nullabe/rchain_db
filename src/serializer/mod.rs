use json::JsonValue;

pub mod block;
pub mod blockchain;
pub mod transaction;

pub trait JsonSerializer {
    fn serialize(&self) -> String;

    fn to_json_value(&self) -> JsonValue;
}

pub trait JsonDeserializer<T: JsonSerializer> {
    fn serialize(object: String) -> T;
}
