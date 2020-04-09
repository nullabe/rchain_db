pub mod block;

pub trait Hasher {
    fn hash(&mut self) -> Option<String>;
}
