use std::error::Error;
use std::fmt;

#[derive(Debug, Default)]
pub struct AddBlockToBlockchainError {
    message: String,
}

impl AddBlockToBlockchainError {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl Error for AddBlockToBlockchainError {}

impl fmt::Display for AddBlockToBlockchainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
