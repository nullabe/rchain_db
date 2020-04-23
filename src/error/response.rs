pub struct ErrorResponse {
    message: String,
    status_code: u16
}

impl ErrorResponse {
    pub fn new(message: String, status_code: u16) -> Self {
        ErrorResponse { message, status_code }
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn get_status_code(&self) -> u16 {
        self.status_code
    }
}
