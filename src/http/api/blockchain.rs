use tide::{Request, Response};

use crate::http::node::Node;
use crate::http::state::BlockchainState;

impl Node {
    pub fn get_blockchain(&mut self) -> &mut Self {
        self.get_server()
            .at("/blockchain")
            .get(|request: Request<BlockchainState>| async move {
                let blockchain = request.state().get_blockchain().lock().unwrap();

                Response::new(200).body_json(&*blockchain).unwrap()
            });

        self
    }
}
