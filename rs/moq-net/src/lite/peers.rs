use serde::{Deserialize, Serialize};
use serde_with::{DeserializeAs, SerializeAs};
use std::time::Duration;
use url::Url;

// The snapshot track carried by each peer
pub const TRACK: &str = "info.json";

pub struct Peers {
	pub enabled: bool,
	pub connect: Url,
	pub listen: crate::transport::Address,
	pub prefix: String,
	pub ice_servers: Option<Vec<String>>,
	pub max: usize,
	pub meta: String,
	pub snapshot: Option<Snapshot>,
	pub iat: String,
}

// Snapshot track
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(default, rename_all = "camelCase")]
#[serde(bound(serialize = "E: Serialize", deserialize = "E: serde::de::DeserializeOwned + Default"))]
#[non_exhaustive]
pub struct Snapshot<E = ()> {
	webrtc: bool,
	presenter_pub_key: Option<String>,
	meta: Option<String>,
	webtransport: Option<BTreeMap<String, String>>,
}

impl Peers {
	pub fn new(
		enabled: bool,
		connect: Url,
		listen: crate::transport::Address,
		prefix: String,
		max: usize,
		meta: serde_json::Value,
	) -> Self {
		Self {
			enabled,
			connect,
			listen,
			prefix,
			ice_servers: None,
			max,
			meta,
		}
	}

	pub fn dial() -> anyhow::Result<()> {
		Ok(())
	}

	pub fn is_selected_pair() -> bool {}

	pub fn gate_peer_connection(&self) -> anyhow::Result<()> {
		// grab peer connection here and then check for gate here
		//let Some(conn) =
		if self.ice_servers.len() <= self.max {}

		Ok(())
	}

	pub fn publish_roster(&self, grant: PeerGrant) -> String {
		for peer in self.ice_servers {
			if let Some(web_transport) = peer.webtransport {
				let urls = todo!("ordered WebTransport URLs from the listener");
			}
			let webrtc_enabled = true;
			let key = grant.key;
			let presenter_pub_key = key.to_public().unwrap();
			let meta = self.meta;

			let snapshot: serde_json::Value = serde_json::from_str(format!("{}"));
		}
	}
}
