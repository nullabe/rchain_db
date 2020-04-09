#[derive(Debug, Clone)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f64) -> Self {
        Self {
            sender,
            receiver,
            amount,
        }
    }

    pub fn get_sender(&self) -> String {
        self.sender.clone()
    }

    pub fn get_receiver(&self) -> String {
        self.receiver.clone()
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }
}
