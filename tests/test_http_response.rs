#[cfg(test)]
pub mod test_http_response {
    use rchain_db::error::response::ErrorResponse;

    #[test]
    fn test_to_json_response() {
        let error = ErrorResponse::new("oops", 404);

        let serde_response = error.to_json_response().unwrap();

        assert_eq!(404, serde_response.status().as_u16());
    }
}
