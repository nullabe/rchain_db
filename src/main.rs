use std::error::Error;

use rchain_db::http::node::Node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Node::run("127.0.0.1", "8080").await?;

    Ok(())
}
