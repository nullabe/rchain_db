use tide::Response;

pub struct JsonResponse {}

impl JsonResponse {
    pub fn response(status: u16, body_string: String) -> Response {
        Response::new(status)
            .body_string(body_string)
            .set_header("Content-Type", "application-json")
    }
}
