use tide::{Request, Response};

use crate::http_api::node::Node;
use crate::http_api::state::BlockchainState;
use crate::error::response::ErrorResponse;
use crate::model::transaction::Transaction;

impl Node {
    pub fn register_routing(&mut self) {
        self.get_blockchain();

        self.post_blocks();

        self.get_transactions_to_process();
        self.post_transactions();
    }

    fn get_blockchain(&mut self) {
        self.get_server()
            .at("/blockchain")
            .get(|request: Request<BlockchainState>| async move {
                let blockchain = request.state().get_blockchain();

                Response::new(200).body_json(blockchain).unwrap()
            });
    }

    fn post_blocks(&mut self) {
        self.get_server()
            .at("/blocks")
            .post(|request: Request<BlockchainState>| async move {
                let blockchain = request.state().get_blockchain();

                Response::new(200).body_json(blockchain).unwrap()
            });
    }

    fn get_transactions_to_process(&mut self) {
        self.get_server()
            .at("/transactions/to_process")
            .get(|request: Request<BlockchainState>| async move {
                let blockchain = request.state().get_blockchain();

                Response::new(200).body_json(&blockchain.get_transactions_to_process()).unwrap()
            });
    }

    fn post_transactions(&mut self) {
        self.get_server()
            .at("/transactions")
            .post(|mut request: Request<BlockchainState>| async move {
                let mut blockchain = request.state().get_blockchain();
                let request_data = request.body_json().await;

                if let Err(err) = request_data {
                    return ErrorResponse::new(format!("Error during request body parsing: {}", err.to_string()), 400)
                        .to_json_response()
                        .unwrap();
                }

                let transaction_request:  serde_json::Result<Transaction> = serde_json::from_value(request_data.unwrap());

                if let Err(err) = transaction_request {
                    return ErrorResponse::new(format!("Error during serialization: {}", err.to_string()), 400)
                        .to_json_response()
                        .unwrap();
                }

                let transaction = transaction_request.unwrap();

                blockchain.add_new_transaction(transaction.get_sender(), transaction.get_receiver(), transaction.get_amount());

                Response::new(200).body_json(&transaction).unwrap()
            });
    }
}
