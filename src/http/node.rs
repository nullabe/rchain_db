use std::error::Error;
use std::sync::{Arc, Mutex};

use tide::Server;

use crate::error::node::UuidNodeError;
use crate::http::state::BlockchainState;
use crate::model::blockchain::Blockchain;
use crate::model::node::{Node, Runner};

impl Runner for Server<BlockchainState> {}

impl Node<Server<BlockchainState>> {
    pub async fn run(host: &str, port: &str) -> Result<(), Box<dyn Error>> {
        let node_uuid = Self::generate_uuid_from_host_mac_address();

        if node_uuid.is_none() {
            return Err(Box::new(UuidNodeError::new(
                "Node uuid is empty".to_string(),
            )));
        }

        let node_uuid = &node_uuid.unwrap();

        let blockchain = Arc::new(Mutex::new(Blockchain::new()));

        let blockchain_state = BlockchainState::from(blockchain, node_uuid);

        let mut node: Node<Server<BlockchainState>> = Node::new(tide::with_state(blockchain_state));

        node.set_uuid(node_uuid).register_routing();

        println!("Node (uuid: {}) created", node_uuid);

        node.server.listen(format!("{}:{}", host, port)).await?;

        Ok(())
    }
}
