use std::net::SocketAddr;

use anyhow::Context;
use axum::http::Method;
use hang::moq_net;
use moq_tokio::RedactedUrl;
use url::Url;

#[derive(usage_rs::Args, Clone)]
#[usage(unknown_flags = "error", args_override_self = false)]
pub struct Args {
	#[usage(name = "p2p-connect", long = "connect", value_name = "URL", group = "p2p-mode")]
	pub connect: Option<Url>,

	#[usage(name = "p2p-listen", long = "listen", value_name = "ADDR", group = "p2p-mode")]
	pub listen: Option<SocketAddr>,

	auth: moq_auth::Grant,
	#[usage(long)]
	enabled: bool,
	prefix: String,
	ice_servers: Option<String>,
	max: usize,
	meta: serde_json::Value,
}

impl Args {
	pub fn validate(&self) -> anyhow::Result<()> {
		Ok(())
	}
}
