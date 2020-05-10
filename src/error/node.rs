use std::error::Error;
use std::fmt;

#[derive(Debug, Default)]
pub struct UuidNodeError {
    message: String,
}

impl UuidNodeError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl Error for UuidNodeError {}

impl fmt::Display for UuidNodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
