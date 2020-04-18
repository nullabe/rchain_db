use std::borrow::BorrowMut;
use std::io::Error;

use tide::Server;

use crate::model::blockchain::Blockchain;

pub struct Node {
    blockchain: Blockchain,
    server: Server<()>,
}

impl Node {
    pub async fn run(host: &str, port: &str) -> Result<(), Error> {
        let blockchain = Blockchain::new();
        let server = tide::new();

        let mut node = Node { blockchain, server };

        node.routing();

        node.server.listen(format!("{}:{}", host, port)).await?;

        Ok(())
    }

    pub fn get_server(&mut self) -> &mut Server<()> {
        self.server.borrow_mut()
    }

    pub fn get_blockchain(&mut self) -> &mut Blockchain {
        self.blockchain.borrow_mut()
    }
}
