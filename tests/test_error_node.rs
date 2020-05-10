#[cfg(test)]
pub mod test_error_node {
    use rchain_db::error::node::UuidNodeError;

    #[test]
    fn test_message() {
        let error = UuidNodeError::new("coucou");

        assert_eq!("coucou", error.message());
    }
}
