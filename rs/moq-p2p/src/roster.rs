use moq_auth::Key;
use moq_net::origin;
use rand::RngExt;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

//use moq_quic::Pair;
use url::Url;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RosterEntry {
	pub id: u64,
	pub alpns: Vec<String>,
	pub webrtc: bool,
	pub presenter_pub_key: Key,
	pub meta: serde_json::Value,
	pub webtransport: Option<WebTransportEntry>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebTransportEntry {
	urls: Vec<Url>,
	#[serde_as(as = "_")]
	cert_fingerprint: u32,
}

pub struct RosterEntryPublisher {
	producer: moq_json::snapshot::Producer<RosterEntry>,
}

impl RosterEntry {
	pub fn new(presenter_pub_key: Key) -> Self {
		Self {
			id: rand::random(),
			alpns: Vec::new(),
			webrtc: false,
			presenter_pub_key,
			meta: serde_json::Value::Null,
			webtransport: None,
		}
	}

	pub fn publish(&self, origin: origin::Producer) {}
}
