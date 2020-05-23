use std::error::Error;

use rchain_db::model::node::Node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Node::run("127.0.0.1", "8880").await?;

    Ok(())
}
