use tide::{Request, Response};

use crate::error::response::ErrorResponse;
use crate::http::state::BlockchainState;
use crate::http::BlockchainServer;
use crate::model::node::Node;

impl Node<BlockchainServer> {
    pub fn post_blocks(&mut self) -> &mut Self {
        self.server
            .at("/blocks")
            .post(|request: Request<BlockchainState>| async move {
                let mut blockchain = request.state().blockchain().lock().unwrap();

                if let Err(err) = blockchain.add_new_block(request.state().node_uuid()) {
                    return ErrorResponse::new(
                        &format!("Error during mining block: {}", err.to_string()),
                        400,
                    )
                    .to_json_response()
                    .unwrap();
                }

                request.state().persist_state(&blockchain);

                Response::new(200)
                    .body_json(blockchain.last_block().unwrap())
                    .unwrap()
            });

        self
    }
}
