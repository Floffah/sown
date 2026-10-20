use bevy_ecs::prelude::Component;

/// Server-assigned network identity, independent of the local Bevy entity ID.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Replicated {
    pub entity_id: u64,
}

/// Peer associated with this entity. The server remains authoritative.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PeerOwner(pub u32);
