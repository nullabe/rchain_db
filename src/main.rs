use rchain_db::http_api::node::Node;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Node::run("127.0.0.1", "8080").await?;

    Ok(())
}
