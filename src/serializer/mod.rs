pub mod block;
pub mod transaction;

pub trait JsonSerializer {
    fn serialize(&self) -> String;
}

pub trait JsonDeserializer<T: JsonSerializer> {
    fn serialize(serialized: String) -> T;
}
