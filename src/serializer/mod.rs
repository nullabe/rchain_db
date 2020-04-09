pub mod block;
pub mod transaction;

pub trait JsonSerializer {
    fn serialize(&self) -> String;
}
