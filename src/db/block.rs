#[derive(Debug, Default)]
pub struct Block {
    hash: Option<String>,
}

impl Block {
    pub fn new() -> Block {
        Block { hash: None }
    }

    pub fn hash(&self) {}
}
