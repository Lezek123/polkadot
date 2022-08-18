// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! All peersets and protocols used for parachains.

use super::ProtocolVersion;
use polkadot_primitives::v2::Hash;
use sc_network::config::{NonDefaultSetConfig, SetConfig};
use std::{
	borrow::Cow,
	collections::{hash_map::Entry, HashMap},
	ops::{Index, IndexMut},
};
use strum::{EnumIter, IntoEnumIterator};

// Only supported protocol versions should be defined here.
const LEGACY_VALIDATION_PROTOCOL_V1: &str = "/polkadot/validation/1";
const LEGACY_COLLATION_PROTOCOL_V1: &str = "/polkadot/collation/1";

/// The main protocol version, currently the same for validation & collation.
const MAIN_PROTOCOL_VERSION: ProtocolVersion = 1;

/// The protocol version for legacy on the wire protocol name, must always be 1.
const LEGACY_PROTOCOL_VERSION: ProtocolVersion = 1;

/// Max notification size is currently constant.
const MAX_NOTIFICATION_SIZE: u64 = 100 * 1024;

/// The peer-sets and thus the protocols which are used for the network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum PeerSet {
	/// The validation peer-set is responsible for all messages related to candidate validation and
	/// communication among validators.
	Validation,
	/// The collation peer-set is used for validator<>collator communication.
	Collation,
}

/// Whether a node is an authority or not.
///
/// Peer set configuration gets adjusted accordingly.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IsAuthority {
	/// Node is authority.
	Yes,
	/// Node is not an authority.
	No,
}

impl PeerSet {
	/// Get `sc_network` peer set configurations for each peerset on the default version.
	///
	/// Those should be used in the network configuration to register the protocols with the
	/// network service.
	pub fn get_info(
		self,
		is_authority: IsAuthority,
		peerset_protocol_names: &PeerSetProtocolNames,
	) -> NonDefaultSetConfig {
		// Networking layer relies on `get_main_name()` being the main name of the protocol
		// for peersets and connection management.
		let protocol = peerset_protocol_names.get_main_name(self);
		let fallback_names = PeerSetProtocolNames::get_fallback_names(self);
		let max_notification_size = self.get_max_notification_size(is_authority);

		match self {
			PeerSet::Validation => NonDefaultSetConfig {
				notifications_protocol: protocol,
				fallback_names,
				max_notification_size,
				set_config: sc_network::config::SetConfig {
					// we allow full nodes to connect to validators for gossip
					// to ensure any `MIN_GOSSIP_PEERS` always include reserved peers
					// we limit the amount of non-reserved slots to be less
					// than `MIN_GOSSIP_PEERS` in total
					in_peers: super::MIN_GOSSIP_PEERS as u32 / 2 - 1,
					out_peers: super::MIN_GOSSIP_PEERS as u32 / 2 - 1,
					reserved_nodes: Vec::new(),
					non_reserved_mode: sc_network::config::NonReservedPeerMode::Accept,
				},
			},
			PeerSet::Collation => NonDefaultSetConfig {
				notifications_protocol: protocol,
				fallback_names,
				max_notification_size,
				set_config: SetConfig {
					// Non-authority nodes don't need to accept incoming connections on this peer set:
					in_peers: if is_authority == IsAuthority::Yes { 100 } else { 0 },
					out_peers: 0,
					reserved_nodes: Vec::new(),
					non_reserved_mode: if is_authority == IsAuthority::Yes {
						sc_network::config::NonReservedPeerMode::Accept
					} else {
						sc_network::config::NonReservedPeerMode::Deny
					},
				},
			},
		}
	}

	/// Get the main protocol version for this peer set.
	///
	/// Networking layer relies on `get_main_version()` being the version
	/// of the main protocol name reported by [`PeerSetProtocolNames::get_main_name()`].
	pub const fn get_main_version(self) -> ProtocolVersion {
		MAIN_PROTOCOL_VERSION
	}

	/// Get the max notification size for this peer set.
	pub fn get_max_notification_size(self, _: IsAuthority) -> u64 {
		MAX_NOTIFICATION_SIZE
	}

	/// Get the peer set label for metrics reporting.
	pub fn get_label(self) -> &'static str {
		match self {
			PeerSet::Validation => "validation",
			PeerSet::Collation => "collation",
		}
	}

	/// Get the protocol label for metrics reporting.
	///
	/// Unfortunately, labels must be static strings, so we must manually cover them
	/// for all protocol versions here.
	pub fn get_protocol_label(self, version: ProtocolVersion) -> Option<&'static str> {
		match (self, version) {
			(PeerSet::Validation, 1) => Some("validation/1"),
			(PeerSet::Collation, 1) => Some("collation/1"),
			_ => None,
		}
	}
}

/// A small and nifty collection that allows to store data pertaining to each peer set.
#[derive(Debug, Default)]
pub struct PerPeerSet<T> {
	validation: T,
	collation: T,
}

impl<T> Index<PeerSet> for PerPeerSet<T> {
	type Output = T;
	fn index(&self, index: PeerSet) -> &T {
		match index {
			PeerSet::Validation => &self.validation,
			PeerSet::Collation => &self.collation,
		}
	}
}

impl<T> IndexMut<PeerSet> for PerPeerSet<T> {
	fn index_mut(&mut self, index: PeerSet) -> &mut T {
		match index {
			PeerSet::Validation => &mut self.validation,
			PeerSet::Collation => &mut self.collation,
		}
	}
}

/// Get `NonDefaultSetConfig`s for all available peer sets, at their default versions.
///
/// Should be used during network configuration (added to [`NetworkConfiguration::extra_sets`])
/// or shortly after startup to register the protocols with the network service.
pub fn peer_sets_info(
	is_authority: IsAuthority,
	peerset_protocol_names: &PeerSetProtocolNames,
) -> Vec<sc_network::config::NonDefaultSetConfig> {
	PeerSet::iter()
		.map(|s| s.get_info(is_authority, &peerset_protocol_names))
		.collect()
}

/// On the wire protocol name to [`PeerSet`] mapping.
#[derive(Clone)]
pub struct PeerSetProtocolNames {
	genesis_hash: Hash,
	fork_id: Option<String>,
	protocols: HashMap<Cow<'static, str>, (PeerSet, ProtocolVersion)>,
}

impl PeerSetProtocolNames {
	/// Construct [`PeerSetProtocols`] using `genesis_hash` and `fork_id`.
	pub fn new(genesis_hash: Hash, fork_id: Option<&str>) -> Self {
		let mut protocols = HashMap::new();
		for protocol in PeerSet::iter() {
			Self::insert_protocol_or_panic(
				&mut protocols,
				Self::generate_name(&genesis_hash, fork_id, protocol, MAIN_PROTOCOL_VERSION),
				protocol,
				MAIN_PROTOCOL_VERSION,
			);
			Self::insert_protocol_or_panic(
				&mut protocols,
				Self::get_legacy_name(protocol),
				protocol,
				LEGACY_PROTOCOL_VERSION,
			)
		}
		Self { genesis_hash, fork_id: fork_id.map(ToOwned::to_owned), protocols }
	}

	/// Helper function to make sure no protocols have the same name.
	fn insert_protocol_or_panic(
		protocols: &mut HashMap<Cow<'static, str>, (PeerSet, ProtocolVersion)>,
		name: Cow<'static, str>,
		protocol: PeerSet,
		version: ProtocolVersion,
	) {
		match protocols.entry(name) {
			Entry::Vacant(entry) => {
				entry.insert((protocol, version));
			},
			Entry::Occupied(entry) => {
				panic!(
					"Protocol {:?} (version {}) has the same on-the-wire name as protocol {:?} (version {}): `{}`.",
					protocol,
					version,
					entry.get().0,
					entry.get().1,
					entry.key(),
				);
			},
		}
	}

	/// Lookup the protocol using its on the wire name.
	pub fn try_get_protocol(&self, name: &Cow<'static, str>) -> Option<(PeerSet, ProtocolVersion)> {
		self.protocols.get(name).map(ToOwned::to_owned)
	}

	/// Get the main protocol name. It's used by the networking for keeping track
	/// of peersets and connections.
	pub fn get_main_name(&self, protocol: PeerSet) -> Cow<'static, str> {
		self.get_name(protocol, MAIN_PROTOCOL_VERSION)
	}

	/// Get the protocol name for specific version.
	pub fn get_name(&self, protocol: PeerSet, version: ProtocolVersion) -> Cow<'static, str> {
		Self::generate_name(&self.genesis_hash, self.fork_id.as_deref(), protocol, version).into()
	}

	/// The protocol name of this protocol based on `genesis_hash` and `fork_id`.
	fn generate_name(
		genesis_hash: &Hash,
		fork_id: Option<&str>,
		protocol: PeerSet,
		version: ProtocolVersion,
	) -> Cow<'static, str> {
		let prefix = if let Some(fork_id) = fork_id {
			format!("/{}/{}", hex::encode(genesis_hash), fork_id)
		} else {
			format!("/{}", hex::encode(genesis_hash))
		};

		let short_name = match protocol {
			PeerSet::Validation => "validation",
			PeerSet::Collation => "collation",
		};

		format!("{}/{}/{}", prefix, short_name, version).into()
	}

	/// Get the legacy protocol name, only `LEGACY_PROTOCOL_VERSION` = 1 is supported.
	fn get_legacy_name(protocol: PeerSet) -> Cow<'static, str> {
		match protocol {
			PeerSet::Validation => VALIDATION_PROTOCOL_V1,
			PeerSet::Collation => COLLATION_PROTOCOL_V1,
		}
		.into()
	}

	/// Get the protocol fallback names. Currently only holds the legacy name
	/// for `LEGACY_PROTOCOL_VERSION` = 1.
	fn get_fallback_names(protocol: PeerSet) -> Vec<Cow<'static, str>> {
		std::iter::once(Self::get_legacy_name(protocol)).collect()
	}
}

#[cfg(test)]
mod tests {
	use super::{Hash, PeerSet, PeerSetProtocolNames};

	#[test]
	fn protocol_names_are_correctly_generated() {
		let genesis_hash = Hash::from([
			122, 200, 116, 29, 232, 183, 20, 109, 138, 86, 23, 253, 70, 41, 20, 85, 127, 230, 60,
			38, 90, 127, 28, 16, 231, 218, 227, 40, 88, 238, 187, 128,
		]);
		let name = PeerSetProtocolNames::generate_name(&genesis_hash, None, PeerSet::Validation, 3);
		let expected =
			"/7ac8741de8b7146d8a5617fd462914557fe63c265a7f1c10e7dae32858eebb80/validation/3";
		assert_eq!(name, expected);

		let name = PeerSetProtocolNames::generate_name(&genesis_hash, None, PeerSet::Collation, 5);
		let expected =
			"/7ac8741de8b7146d8a5617fd462914557fe63c265a7f1c10e7dae32858eebb80/collation/5";
		assert_eq!(name, expected);

		let fork_id = Some("test-fork");
		let name =
			PeerSetProtocolNames::generate_name(&genesis_hash, fork_id, PeerSet::Validation, 7);
		let expected =
			"/7ac8741de8b7146d8a5617fd462914557fe63c265a7f1c10e7dae32858eebb80/test-fork/validation/7";
		assert_eq!(name, expected);

		let name =
			PeerSetProtocolNames::generate_name(&genesis_hash, fork_id, PeerSet::Collation, 11);
		let expected =
			"/7ac8741de8b7146d8a5617fd462914557fe63c265a7f1c10e7dae32858eebb80/test-fork/collation/11";
		assert_eq!(name, expected);
	}

	#[test]
	fn all_protocol_names_are_known() {
		let genesis_hash = Hash::from([
			122, 200, 116, 29, 232, 183, 20, 109, 138, 86, 23, 253, 70, 41, 20, 85, 127, 230, 60,
			38, 90, 127, 28, 16, 231, 218, 227, 40, 88, 238, 187, 128,
		]);
		let protocol_names = PeerSetProtocolNames::new(genesis_hash, None);

		let validation_main =
			"/7ac8741de8b7146d8a5617fd462914557fe63c265a7f1c10e7dae32858eebb80/validation/1";
		assert_eq!(
			protocol_names.try_get_protocol(&validation_main.into()),
			Some((PeerSet::Validation, 1)),
		);

		let validation_legacy = "/polkadot/validation/1";
		assert_eq!(
			protocol_names.try_get_protocol(&validation_legacy.into()),
			Some((PeerSet::Validation, 1)),
		);

		let collation_main =
			"/7ac8741de8b7146d8a5617fd462914557fe63c265a7f1c10e7dae32858eebb80/collation/1";
		assert_eq!(
			protocol_names.try_get_protocol(&collation_main.into()),
			Some((PeerSet::Collation, 1)),
		);

		let collation_legacy = "/polkadot/collation/1";
		assert_eq!(
			protocol_names.try_get_protocol(&collation_legacy.into()),
			Some((PeerSet::Collation, 1)),
		);
	}
}
