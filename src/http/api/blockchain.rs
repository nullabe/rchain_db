use tide::{Request, Response, Server};

use crate::http::state::BlockchainState;
use crate::model::node::Node;

impl Node<Server<BlockchainState>> {
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
