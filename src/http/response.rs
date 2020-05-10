use tide::Response;

use crate::error::response::ErrorResponse;

impl ErrorResponse {
    pub fn to_json_response(&self) -> serde_json::Result<Response> {
        Response::new(self.status_code()).body_json(self)
    }
}
