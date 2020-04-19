#[cfg(test)]
pub mod test_serializer_blockchain {
    use rchain_db::model::blockchain::Blockchain;
    use rchain_db::serializer::JsonSerializer;

    #[test]
    fn test_serialize() {
        let mut blockchain = Blockchain::new();

        blockchain.add_new_transaction(String::from("s1"), String::from("r1"), 66.6);
        blockchain.add_new_block().ok();

        assert_eq!(
            String::from("{\"count\":1,\"blocks\":[{\"hash\":\"910e768724bbd3f48320fd25c81b0e0bfcb57682b849d4f2a6be3750a96ac5e6\",\"index\":0,\"transactions\":[{\"sender\":\"s1\",\"receiver\":\"r1\",\"amount\":66.6}],\"algorithm_proof\":0,\"previous_block_hash\":\"\"}]}"),
            blockchain.serialize()
        );
    }
}
