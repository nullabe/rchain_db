#[derive(Debug, Clone)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

impl Transaction {
    pub fn new(sender: &str, receiver: &str, amount: f64) -> Self {
        Self {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
        }
    }

    pub fn sender(&self) -> String {
        self.sender.clone()
    }

    pub fn receiver(&self) -> String {
        self.receiver.clone()
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }
}
