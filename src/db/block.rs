#[derive(Debug)]
pub struct Block {
    hash: Option<String>,
}

impl Block {
    pub fn new() -> Self {
        Block { hash: None }
    }

    pub fn hash(&self) {}
}

impl Default for Block {
    fn default() -> Self {
        Block::new()
    }
}
