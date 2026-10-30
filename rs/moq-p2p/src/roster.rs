use moq_auth::Key;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as};
use moq_net::origin;
//use moq_quic::Pair;
use url::Url;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RosterEntry {
	alpns: Vec<String>,
	webrtc: bool,
	presenter_pub_key: Key,
	meta: serde_json::Value,
	webtransport: Option<WebTransportEntry>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebTransportEntry {
	urls: Vec<Url>,
	#[serde_as(as ="_")]
	cert_fingerprint: u32,
}

impl RosterEntry {
	pub fn new(presenter_pub_key: Key) -> Self {
		Self {
			alpns: Vec::new(),
			webrtc: false,
			presenter_pub_key,
			meta: serde_json::Value::Null,
			webtransport: None,
		}
	}

	pub fn publish(&self, origin: origin::Producer) {}
}
