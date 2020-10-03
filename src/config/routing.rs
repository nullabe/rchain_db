use crate::http::BlockchainServer;
use crate::model::node::Node;

impl Node<BlockchainServer> {
    pub fn register_routes(&mut self) {
        self.get_blockchain()
            .get_transactions_to_process()
            .post_blocks()
            .post_nodes()
            .post_transactions();
    }
}
