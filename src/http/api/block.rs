use tide::{Request, Response, Server};

use crate::error::response::ErrorResponse;
use crate::http::state::BlockchainState;
use crate::model::node::Node;

impl Node<Server<BlockchainState>> {
    pub fn post_blocks(&mut self) -> &mut Self {
        self.server
            .at("/blocks")
            .post(|request: Request<BlockchainState>| async move {
                let mut blockchain = request.state().blockchain().lock().unwrap();

                if let Err(err) = blockchain.add_new_block(request.state().node_uuid()) {
                    return ErrorResponse::new(
                        format!("Error during mining block: {}", err.to_string()),
                        400,
                    )
                    .to_json_response()
                    .unwrap();
                }

                Response::new(200)
                    .body_json(blockchain.last_block().unwrap())
                    .unwrap()
            });

        self
    }
}
