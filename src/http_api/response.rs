use crate::error::response::ErrorResponse;
use tide::Response;

impl ErrorResponse {
    pub fn to_json_response(&self) -> serde_json::Result<Response> {
        Response::new(self.get_status_code()).body_json(self)
    }
}
