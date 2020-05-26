use std::error::Error;

use crate::error::node::UuidNodeError;
use crate::http::state::BlockchainState;
use crate::http::BlockchainServer;
use crate::model::node::{Node, Runner};
use crate::storage::file::blockchain::BlockchainFileStorage;

impl Runner for BlockchainServer {}

impl Node<BlockchainServer> {
    pub async fn run(host: &str, port: &str) -> Result<(), Box<dyn Error>> {
        let node_uuid = Self::generate_uuid_from_host_mac_address();

        if node_uuid.is_none() {
            return Err(Box::new(UuidNodeError::new("Node uuid is empty")));
        }

        let node_uuid = &node_uuid.unwrap();

        let blockchain_state = BlockchainState::from(BlockchainFileStorage, node_uuid);

        let mut node: Node<BlockchainServer> = Node::new(tide::with_state(blockchain_state));

        node.set_uuid(node_uuid).register_routes();

        println!("Node (uuid: {}) created", node_uuid);

        node.server.listen(format!("{}:{}", host, port)).await?;

        Ok(())
    }
}
