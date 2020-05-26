use tide::Server;

use crate::http::state::BlockchainState;

pub mod api;
pub mod node;
pub mod response;
pub mod state;

pub type BlockchainServer = Server<BlockchainState>;
