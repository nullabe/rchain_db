#[cfg(test)]
pub mod test_serializer_blockchain {
    use serde::Serialize;
    use serde_json::value::Serializer;

    use rchain_db::model::blockchain::Blockchain;

    #[test]
    fn test_serialize() {
        let mut blockchain = Blockchain::new();

        blockchain.add_new_transaction(String::from("s1"), String::from("r1"), 66.6);
        blockchain.add_new_block().ok();

        assert_eq!(
            String::from("{\"blocks\":[{\"algorithm_proof\":0,\"hash\":\"e5fd528412f1210a2baf296a849ba880ee212a49a3f6275d8446afef6c2c1f48\",\"index\":0,\"previous_block_hash\":\"\",\"transactions\":[{\"amount\":66.6,\"receiver\":\"r1\",\"sender\":\"s1\"}]}],\"count\":1}"),
            blockchain.serialize(Serializer).unwrap().to_string()
        );
    }
}
