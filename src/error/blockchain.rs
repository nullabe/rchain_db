use std::error::Error;
use std::fmt;

#[derive(Debug, Default)]
pub struct AddBlockToBlockchainError {
    message: String,
}

impl AddBlockToBlockchainError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
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

#[derive(Debug, Default)]
pub struct RegisterNodeToBlockchainError {
    message: String,
}

impl RegisterNodeToBlockchainError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl Error for RegisterNodeToBlockchainError {}

impl fmt::Display for RegisterNodeToBlockchainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
