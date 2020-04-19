use tide::Request;

use crate::http_api::node::Node;
use crate::http_api::response::JsonResponse;
use crate::http_api::state::BlockchainState;
use crate::serializer::JsonSerializer;

impl Node {
    pub fn routing(&mut self) {
        self.root_action();
    }

    fn root_action(&mut self) {
        self.get_server()
            .at("/")
            .get(|request: Request<BlockchainState>| async move {
                let blockchain = request.state().get_blockchain();

                JsonResponse::response(200, blockchain.serialize())
            });
    }
}
