#[cfg(test)]
pub mod test_error_response {
    use rchain_db::error::response::ErrorResponse;

    #[test]
    fn test_message() {
        let error = ErrorResponse::new("oops", 404);

        assert_eq!("oops", error.message());
    }

    #[test]
    fn test_status_code() {
        let error = ErrorResponse::new("oops", 404);

        assert_eq!(404, error.status_code());
    }
}
