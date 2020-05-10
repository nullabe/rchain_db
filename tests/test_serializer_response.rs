#[cfg(test)]
pub mod test_serializer_response {
    use serde::Serialize;
    use serde_json::value::Serializer;

    use rchain_db::error::response::ErrorResponse;

    #[test]
    fn test_serialize() {
        let error = ErrorResponse::new("oops", 404);

        assert_eq!(
            String::from("{\"message\":\"oops\",\"status_code\":404}"),
            error.serialize(Serializer).unwrap().to_string()
        );
    }
}
