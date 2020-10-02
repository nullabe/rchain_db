#[cfg(test)]
pub mod test_serializer_node {
    use serde::Serialize;
    use serde_json::value::Serializer;

    use rchain_db::model::node::{NeighbourServer, Node};

    #[test]
    fn test_serialize() {
        let mut node = Node::new(NeighbourServer);

        node.set_url("https://twiiter.com/nullabe_music");
        node.set_uuid("huhu id");

        assert_eq!(
            String::from("{\"url\":\"https://twiiter.com/nullabe_music\",\"uuid\":\"huhu id\"}"),
            node.serialize(Serializer).unwrap().to_string()
        );
    }

    #[test]
    fn test_deserialize() {
        let serialized_node: Node<NeighbourServer> = serde_json::from_str(
            "{\"uuid\":\"huhu id\",\"url\":\"https://twitter.com/nullabe_music\"}",
        )
        .unwrap();

        assert_eq!(
            "https://twitter.com/nullabe_music",
            serialized_node.url().unwrap()
        );
        assert_eq!("huhu id", serialized_node.uuid().unwrap());
    }
}
