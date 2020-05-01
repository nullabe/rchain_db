use std::error::Error;
use std::sync::{Arc, Mutex};

use tide::Server;

use crate::crypto::Hasher;
use crate::error::node::UuidNodeError;
use crate::http::state::BlockchainState;
use crate::model::blockchain::Blockchain;
use crate::model::node::{Node, Runner};

impl Runner for Server<BlockchainState> {}

impl Node<Server<BlockchainState>> {
    pub async fn run(host: &str, port: &str) -> Result<(), Box<dyn Error>> {
        let blockchain = Arc::new(Mutex::new(Blockchain::new()));

        let blockchain_state = BlockchainState::from(blockchain);

        let mut node: Node<Server<BlockchainState>> = Node::new(tide::with_state(blockchain_state));

        let uuid = node.hash();

        if uuid.is_none() {
            return Err(Box::new(UuidNodeError::new(
                "Node uuid is empty".to_string(),
            )));
        }

        node.set_uuid(uuid);

        node.register_routing();

        println!("Node (uuid: {}) created", node.get_uuid().unwrap());

        node.server.listen(format!("{}:{}", host, port)).await?;

        Ok(())
    }
}
