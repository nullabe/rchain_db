use tide::{Request, Response};

use crate::http::state::BlockchainState;
use crate::http::BlockchainServer;
use crate::model::node::Node;

impl Node<BlockchainServer> {
    pub fn get_blockchain(&mut self) -> &mut Self {
        self.server
            .at("/blockchain")
            .get(|request: Request<BlockchainState>| async move {
                let blockchain = request.state().blockchain().lock().unwrap();

                Response::new(200).body_json(&*blockchain).unwrap()
            });

        self
    }
}
