# moq-net lite: P2P slice

## Overview

Scope: only the P2P additions to the moq-lite module, not the rest of
`lite/` (sessions, announce, subscribe, fetch, probe, and so on). Covers
`peers.rs`, the empty `p2p.rs`, and `PeersSetup` in `setup.rs`. `mod peers;`
and `PeersSetup` are gated on the `p2p` feature in `rs/moq-net/Cargo.toml`.

The quest places P2P policy in `Peers` in [`@moq/p2p`](../../../../js/p2p/EGM.md)
with a native mirror in [moq-cli](../../../moq-cli/src/p2p/EGM.md). Its only
`moq-lite` wire work is the AUTH additions in
[Peer grants](../../../../quest/m2/p2p/peer-grant.md) (`AUTH_CNF`,
`AUTH_KEYS { Jwks }`, `AUTH_PEER { Grant }`, possession proof), which land in
whichever lite wip is current, together with
`drafts/draft-lcurley-moq-lite.md` and `js/net`.

Status: work in progress; `cargo check -p moq-net --features p2p` fails with
18 errors across `peers.rs` and `setup.rs` (missing `serde_json`,
`serde_with`, and `anyhow` dependencies, unimported `Url`, `BTreeMap`, and
`PeerGrant`, an empty `is_selected_pair` body, a `Peers` literal missing
`snapshot`). Default builds without `p2p` compile. `mod peers;` is declared in `mod.rs`; `p2p.rs` is
not.

## Interface

- `peers::TRACK = "info.json"`: the roster snapshot track name.
- `peers::Peers` (WIP): `enabled`, `connect: Url`,
  `listen: crate::transport::Address`, `prefix`, `ice_servers`, `max`, `meta`,
  `snapshot`; methods `new`, `dial`, `is_selected_pair`,
  `gate_peer_connection`, `publish_roster`.
- `peers::Snapshot<E>` (WIP, camelCase serde): `webrtc`,
  `presenter_pub_key`, `meta`, `webtransport`.
- `setup::PeersSetup` (crate-private, WIP): a `kio::Shared` slot holding the
  same policy fields plus `Option<Setup>`.
- Related WIP outside this directory: `moq_auth::PeerGrant`
  (`rs/moq-auth/src/grant.rs`) and the empty `Refresh::auth_keys` and
  `Refresh::auth_peers` in `rs/moq-relay/src/auth/refresh.rs`.

## Invariants

Plan invariants (from [Signaling and policy](../../../../quest/m2/p2p/signal.md)
and [Peer grants](../../../../quest/m2/p2p/peer-grant.md)):

- Roster entry: `<prefix><id>` with an `info.json` snapshot carrying accepted
  moq ALPNs, `webrtc: true`, the presenter public key, the application's opaque
  `meta`, and for native peers an ordered WebTransport URL list with the
  listener fingerprint plus the ICE+QUIC field. What names `<id>` is open
  ([Plan P2P identity](../../../../quest/m2/p2p/identity-plan.md)).
- STUN gate: a new negotiation gets `iceServers` only while the roster holds
  `max` peers or fewer; host and mDNS candidates are always gathered;
  established pairs are never closed by the gate.
- A peer grant is a distinct type from `Claims`, signed only with an
  asymmetric key (ES256 or EdDSA), carrying publish and subscribe patterns,
  the presenter key as `cnf`, `iat`, and a short `exp`, and no routing
  identity. An HS256-only relay issues none.
- Routing identities stay declared and unverified; CDN to direct handover
  resumes only across a matching explicit epoch.

Plan drift in the current stubs:

- Layering: `Peers` holds a dial URL, a listen address, and ICE servers, which
  are transport and application policy. `moq-net` lite is the wire and session
  layer; the quest keeps this in `@moq/p2p` and the native client/CLI.
- `gate_peer_connection` compares `ice_servers.len()` to `max`; the gate is on
  roster size.
- `ice_servers: Option<Vec<String>>` adds a `None` state the quest does not
  have; an empty list already means LAN only.
- `meta` is typed `String` in `Peers` and `Snapshot` but passed a
  `serde_json::Value`; the quest calls it an opaque JSON value.
- `webtransport: Option<BTreeMap<String, String>>` is unordered; the quest
  requires an ordered URL list plus one fingerprint.
- `PeerGrant::new` generates a key and discards it, and `cnf` defaults: `cnf`
  must be the client's presenter key announced on `AUTH_CNF`, not one the
  relay mints. `exp` is a `Duration`, and `iat` is missing.

## Side Effects

- Planned: publishing the roster broadcast and per-pair `signal` tracks into
  the shared origin; new AUTH stream messages on relay and direct sessions.
- Current code: none.

## Verification

- Run: `cargo test -p moq-net --features p2p`
- Run: `just drafts check` (once the AUTH messages are drafted)
- Run: `just test interop --all` (wire change)
