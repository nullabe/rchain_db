use crate::model::node::{NeighbourServer, Node};

pub mod block;
pub mod blockchain;
pub mod node;
pub mod proof_of_work;
pub mod transaction;

type NeighbourhoodNodes = Vec<Node<NeighbourServer>>;
