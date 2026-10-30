use moq_pattern::Patterns;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use moq_net::origin;

use moq_auth::{Jwk, Key};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Peers {
	enabled: bool,
	prefix: String,
	ice_servers: Vec<String>,
	max: usize,
	meta: serde_json::Value,
}

impl Default for Peers {
	fn default() -> Self {
		Peers {
			enabled: false,
			prefix: ".p2p/".into(),
			ice_servers: Vec::new(),
			max: 12,
			meta: serde_json::Value::Null,


		}

	}
}

impl Peers {
	pub fn publish(&self, origin: origin::Producer) {}
}
