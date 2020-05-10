#[cfg(test)]
pub mod test_model_node {
    use rchain_db::model::node::{Node, Runner};

    #[test]
    fn test_new() {
        let node = Node::new(ServerMock {
            name: "server".to_string(),
        });

        assert_eq!("server".to_string(), node.server.name);
    }

    #[test]
    fn test_get_uuid() {
        let mut node = Node::new(ServerMock {
            name: "server".to_string(),
        });

        node.set_uuid("hey");

        assert_eq!("hey", node.get_uuid().unwrap());
    }

    struct ServerMock {
        pub name: String,
    }

    impl Runner for ServerMock {}
}
