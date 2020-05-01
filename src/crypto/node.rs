use crypto::sha2::Sha256;
use mac_address::get_mac_address;

use crate::crypto::Hasher;
use crate::http::node::Node;
use crypto::digest::Digest;

impl Hasher for Node {
    fn hash(&mut self) -> Option<String> {
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
