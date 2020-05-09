use tide::Server;

use crate::http::state::BlockchainState;
use crate::model::node::Node;

impl Node<Server<BlockchainState>> {
    pub fn register_routes(&mut self) {
        self.get_blockchain()
            .get_transactions_to_process()
            .post_blocks()
            .post_transactions();
    }
}
