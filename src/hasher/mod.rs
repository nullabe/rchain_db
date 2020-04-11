pub mod block;
pub mod proof_of_work;

pub trait ModelHasher {
    fn hash(&mut self) -> Option<String>;
}

pub trait ValidatorHasher {
    fn validate(&self, to_validate: &str, difficulty: &str) -> bool;
}
