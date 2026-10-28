use moq_pattern::Patterns;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{Jwk, Key};

/// What a session may serve on a direct peer connection, signed by the relay and bound
/// to the presenter's public key.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "Wire", into = "Wire")]
#[non_exhaustive]
pub struct PeerGrant {
	/// The root for the publish/subscribe patterns below.
	pub root: String,

	/// Patterns the presenter may publish, relative to the root.
	pub publish: Patterns,

	/// Patterns the presenter may subscribe to, relative to the root.
	pub subscribe: Patterns,

	/// The presenter's public key (`cnf`), proven on the direct session.
	pub cnf: Key,

	/// When the grant expires (`exp`). Required, since nothing else bounds a grant a peer holds.
	pub expires: moq_clock::Wall,

	/// When the grant was issued (`iat`). Purely informational; nothing enforces it.
	pub issued: Option<moq_clock::Wall>,
}

impl PeerGrant {
	/// A grant bound to the presenter key `cnf` that expires at `expires`, naming no paths yet.
	pub fn new(cnf: Key, expires: moq_clock::Wall) -> Self {
		Self {
			root: String::new(),
			publish: Patterns::default(),
			subscribe: Patterns::default(),
			cnf,
			expires,
			issued: None,
		}
	}

	/// Set the root that the publish/subscribe patterns are relative to.
	pub fn with_root(mut self, root: impl Into<String>) -> Self {
		self.root = root.into();
		self
	}

	/// Grant publish access to these patterns, relative to the root.
	pub fn with_publish(mut self, patterns: impl IntoIterator<Item = moq_pattern::Pattern>) -> Self {
		self.publish = patterns.into_iter().collect();
		self
	}

	/// Grant subscribe access to these patterns, relative to the root.
	pub fn with_subscribe(mut self, patterns: impl IntoIterator<Item = moq_pattern::Pattern>) -> Self {
		self.subscribe = patterns.into_iter().collect();
		self
	}

	/// Record when the grant was issued.
	pub fn with_issued(mut self, at: impl Into<Option<moq_clock::Wall>>) -> Self {
		self.issued = at.into();
		self
	}

	/// Refuse a grant that names nothing, or whose `cnf` is not a public asymmetric key.
	pub fn validate(&self) -> crate::Result<()> {
		if self.publish.is_empty() && self.subscribe.is_empty() {
			return Err(crate::Error::UselessToken);
		}
		// A private cnf would hand the presenter's secret to every verifier.
		if self.cnf.is_private() {
			return Err(crate::Error::PrivateCnf);
		}
		Ok(())
	}
}

/// The JWT payload of a [`PeerGrant`].
#[serde_with::skip_serializing_none]
#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Wire {
	#[serde(default, skip_serializing_if = "String::is_empty")]
	root: String,
	#[serde(default, skip_serializing_if = "Patterns::is_empty")]
	publish: Patterns,
	#[serde(default, skip_serializing_if = "Patterns::is_empty")]
	subscribe: Patterns,
	cnf: Cnf,
	#[serde_as(as = "moq_clock::serde::UnixSeconds")]
	exp: moq_clock::Wall,
	#[serde_as(as = "Option<moq_clock::serde::UnixSeconds>")]
	#[serde(default)]
	iat: Option<moq_clock::Wall>,
}

// RFC 7800 confirmation claim: `{"cnf": {"jwk": {...}}}`.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Cnf {
	jwk: Jwk,
}

impl From<PeerGrant> for Wire {
	fn from(grant: PeerGrant) -> Self {
		Self {
			root: grant.root,
			publish: grant.publish,
			subscribe: grant.subscribe,
			cnf: Cnf {
				jwk: grant.cnf.export(),
			},
			exp: grant.expires,
			iat: grant.issued,
		}
	}
}

impl TryFrom<Wire> for PeerGrant {
	type Error = crate::Error;

	fn try_from(wire: Wire) -> crate::Result<Self> {
		Ok(Self {
			root: wire.root,
			publish: wire.publish,
			subscribe: wire.subscribe,
			cnf: wire.cnf.jwk.import()?,
			expires: wire.exp,
			issued: wire.iat,
		})
	}
}
