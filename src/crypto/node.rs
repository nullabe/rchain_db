use crypto::digest::Digest;
use crypto::sha2::Sha256;
use mac_address::get_mac_address;
use tide::Server;

use crate::http::state::BlockchainState;
use crate::model::node::Node;

impl Node<Server<BlockchainState>> {
    pub fn generate_uuid_from_host_mac_address() -> Option<String> {
        let mut hasher = Sha256::new();
        let mac_address = get_mac_address();

        let mac_address = match mac_address {
            Ok(mac_address) => mac_address,

            Err(_) => return None,
        };

        mac_address?;

        hasher.input_str(&mac_address.unwrap().to_string());

        Some(hasher.result_str())
    }
}
