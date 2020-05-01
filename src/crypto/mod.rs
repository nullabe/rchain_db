pub mod block;
pub mod node;
pub mod proof_of_work;

pub trait Hasher {
    fn hash(&mut self) -> Option<String>;
}

pub trait Validator {
    fn validate(&self, to_validate: &str, difficulty: &str) -> bool;
}
