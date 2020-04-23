use std::borrow::BorrowMut;
use std::io::Error;

use tide::Server;

use crate::http_api::state::BlockchainState;
use crate::model::blockchain::Blockchain;

pub struct Node {
    server: Server<BlockchainState>,
}

impl Node {
    pub async fn run(host: &str, port: &str) -> Result<(), Error> {
        let blockchain_state = BlockchainState::from(Blockchain::new());

        let server = tide::with_state(blockchain_state);

        let mut node = Node { server };

        node.register_routing();

        node.server.listen(format!("{}:{}", host, port)).await?;

        Ok(())
    }

    pub fn get_server(&mut self) -> &mut Server<BlockchainState> {
        self.server.borrow_mut()
    }
}
