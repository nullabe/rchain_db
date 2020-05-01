use crate::http::node::Node;

impl Node {
    pub fn register_routing(&mut self) {
        self.get_blockchain()
            .get_transactions_to_process()
            .post_blocks()
            .post_transactions();
    }
}
