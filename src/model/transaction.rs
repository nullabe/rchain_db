use crate::model::block::Block;

#[derive(Debug)]
pub struct Transaction<'blockchain> {
    block: Option<&'blockchain Block<'blockchain>>,
    sender: String,
    receiver: String,
    amount: f64,
}

impl<'blockchain> Transaction<'blockchain> {
    pub fn new(sender: String, receiver: String, amount: f64) -> Self {
        Transaction {
            block: None,
            sender,
            receiver,
            amount,
        }
    }
}
