use crate::model::blockchain::Blockchain;

#[derive(Debug)]
pub struct Block<'blockchain> {
    blockchain: &'blockchain Blockchain<'blockchain>,
    hash: Option<String>,
    index: usize,
}

impl<'blockchain> Block<'blockchain> {
    pub fn new(blockchain: &'blockchain Blockchain) -> Self {
        let mut block = Block {
            blockchain,
            hash: None,
            index: 0,
        };

        block.hash();

        block
    }

    fn hash(&mut self) {
        self.hash = Some(String::from(""));
    }
}
