use tide::{Request, Response};

use crate::error::response::ErrorResponse;
use crate::http::node::Node;
use crate::http::state::BlockchainState;

impl Node {
    pub fn post_blocks(&mut self) -> &mut Self {
        self.get_server()
            .at("/blocks")
            .post(|request: Request<BlockchainState>| async move {
                let mut blockchain = request.state().get_blockchain().lock().unwrap();

                if blockchain.get_transactions_to_process().is_empty() {
                    return ErrorResponse::new("No new transactions to process".to_string(), 400)
                        .to_json_response()
                        .unwrap();
                }

                if let Err(err) = blockchain.add_new_block() {
                    return ErrorResponse::new(
                        format!("Error during mining block: {}", err.to_string()),
                        400,
                    )
                    .to_json_response()
                    .unwrap();
                }

                Response::new(200)
                    .body_json(blockchain.get_blocks().last().unwrap())
                    .unwrap()
            });

        self
    }
}
