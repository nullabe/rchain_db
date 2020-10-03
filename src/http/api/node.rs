use tide::{Request, Response};

use crate::crypto::Sha256Blockchain;
use crate::error::response::ErrorResponse;
use crate::http::state::BlockchainState;
use crate::http::BlockchainServer;
use crate::model::consensus;
use crate::model::node::{NeighbourServer, Node};

impl Node<BlockchainServer> {
    pub fn post_nodes(&mut self) -> &mut Self {
        self.server
            .at("/nodes")
            .post(|mut request: Request<BlockchainState>| async move {
                let request_data = request.body_string().await;

                let mut blockchain = request.state().blockchain().lock().unwrap();

                if let Err(err) = request_data {
                    return ErrorResponse::new(
                        &format!("Error during request body parsing: {}", err.to_string()),
                        400,
                    )
                    .to_json_response()
                    .unwrap();
                }

                let node_request: serde_json::Result<Node<NeighbourServer>> =
                    serde_json::from_str(&request_data.unwrap());

                if let Err(err) = node_request {
                    return ErrorResponse::new(
                        &format!("Error during serialization: {}", err.to_string()),
                        400,
                    )
                    .to_json_response()
                    .unwrap();
                }

                let node = node_request.unwrap();

                if let Err(err) =
                    blockchain.add_neighbour_node(&node.uuid().unwrap(), &node.url().unwrap())
                {
                    return ErrorResponse::new(err.message(), 400)
                        .to_json_response()
                        .unwrap();
                }

                request.state().persist_state(&blockchain);

                Response::new(200).body_json(&node).unwrap()
            });

        self
    }

    pub fn post_nodes_resolve(&mut self) -> &mut Self {
        self.server
            .at("/nodes/resolve")
            .post(|request: Request<BlockchainState>| async move {
                let mut blockchain = request.state().blockchain().lock().unwrap();

                for node in &blockchain.neighbour_nodes() {
                    let node_blockchain_endpoint = node.url().unwrap().to_string() + "/blockchain";

                    let body = reqwest::blocking::get(&node_blockchain_endpoint)
                        .unwrap()
                        .text()
                        .unwrap();

                    let node_blockchain: Sha256Blockchain = serde_json::from_str(&body).unwrap();

                    if !consensus::is_blockchain_authoritative(
                        &node_blockchain,
                        blockchain.blocks().len(),
                    ) {
                        continue;
                    }

                    blockchain.replace_blocks(node_blockchain.blocks());
                    blockchain
                        .replace_transactions_to_process(node_blockchain.transactions_to_process());
                }

                Response::new(200)
                    .body_json(blockchain.last_block().unwrap())
                    .unwrap()
            });

        self
    }
}
