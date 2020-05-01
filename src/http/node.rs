use std::borrow::BorrowMut;
use std::error::Error;
use std::sync::{Arc, Mutex};

use tide::Server;

use crate::crypto::Hasher;
use crate::error::node::UuidNodeError;
use crate::http::state::BlockchainState;
use crate::model::blockchain::Blockchain;

pub struct Node {
    uuid: Option<String>,
    server: Server<BlockchainState>,
}

impl Node {
    pub async fn run(host: &str, port: &str) -> Result<(), Box<dyn Error>> {
        let blockchain = Arc::new(Mutex::new(Blockchain::new()));

        let blockchain_state = BlockchainState::from(blockchain);

        let server = tide::with_state(blockchain_state);

        let mut node = Node { server, uuid: None };

        node.uuid = node.hash();

        if node.uuid.is_none() {
            return Err(Box::new(UuidNodeError::new(
                "Node uuid is empty".to_string(),
            )));
        }

        node.register_routing();

        println!("Node (uuid: {}) created", node.uuid.unwrap());

        node.server.listen(format!("{}:{}", host, port)).await?;

        Ok(())
    }

    pub fn get_server(&mut self) -> &mut Server<BlockchainState> {
        self.server.borrow_mut()
    }
}
