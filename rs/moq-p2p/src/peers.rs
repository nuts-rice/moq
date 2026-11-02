use crate::roster::RosterEntry;
use moq_net::origin;
use moq_pattern::Patterns;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

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
	async fn publish(&self, origin: moq_net::origin::Producer, roster_entry: RosterEntry) -> anyhow::Result<()> {
		if !self.enabled {
			return Ok(());
		}
		let prefix = self.prefix.clone();
		let ice_servers = self.ice_servers.clone();
		let roster_id = roster_entry.id.clone();

		let client = moq_tokio::connect::Config::default().init(Default::default())?;
		let url = url::Url::parse("{prefix}{roster_id}")?;

		let reconnect = client.with_publisher(&origin).connect(url);

		Ok(reconnect.closed().await?)
	}
}
