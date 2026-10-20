use std::collections::BTreeSet;

use anyhow::{Result, ensure};
use protocol::v1::{EntitySnapshot, PeerSnapshot};

/// Reliable baseline received during Join, including the assigned local player.
#[derive(Debug)]
pub struct JoinedWorld {
    pub peer_id: u32,
    pub player_entity_id: u64,
    pub server_tick: u64,
    pub peers: Vec<PeerSnapshot>,
    pub entities: Vec<EntitySnapshot>,
}

impl JoinedWorld {
    /// Checks identities, ownership, and transforms before applying the baseline.
    ///
    /// # Errors
    /// Returns an error for duplicate IDs, invalid entities, or a missing local player.
    pub fn validate(&self) -> Result<()> {
        let mut peers = BTreeSet::new();
        for peer in &self.peers {
            ensure!(
                peer.peer_id != 0 && peers.insert(peer.peer_id),
                "Invalid or duplicate peer ID"
            );
        }
        ensure!(
            peers.contains(&self.peer_id),
            "Initial snapshot is missing the local peer"
        );
        let mut entities = BTreeSet::new();
        for entity in &self.entities {
            validate_entity(entity)?;
            ensure!(entities.insert(entity.entity_id), "Duplicate entity ID");
            ensure!(
                peers.contains(&entity.owner_peer_id),
                "Unknown entity owner"
            );
        }
        ensure!(
            self.entities
                .iter()
                .any(|entity| entity.entity_id == self.player_entity_id
                    && entity.owner_peer_id == self.peer_id),
            "Initial snapshot is missing the assigned player"
        );
        Ok(())
    }
}

/// Checks a network entity's identity and complete finite transform.
///
/// # Errors
/// Returns an error for an invalid ID, missing transform fields, or invalid numbers.
pub fn validate_entity(entity: &EntitySnapshot) -> Result<()> {
    ensure!(
        entity.entity_id != 0 && entity.owner_peer_id != 0,
        "Invalid entity identity"
    );
    let transform = entity
        .transform
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Missing transform"))?;
    let translation = transform
        .translation
        .ok_or_else(|| anyhow::anyhow!("Missing translation"))?;
    let rotation = transform
        .rotation
        .ok_or_else(|| anyhow::anyhow!("Missing rotation"))?;
    let scale = transform
        .scale
        .ok_or_else(|| anyhow::anyhow!("Missing scale"))?;
    ensure!(
        [
            translation.x,
            translation.y,
            translation.z,
            rotation.x,
            rotation.y,
            rotation.z,
            rotation.w,
            scale.x,
            scale.y,
            scale.z,
        ]
        .iter()
        .all(|value| value.is_finite()),
        "Non-finite transform"
    );
    let norm = rotation.x * rotation.x
        + rotation.y * rotation.y
        + rotation.z * rotation.z
        + rotation.w * rotation.w;
    ensure!((norm - 1.0).abs() < 0.001, "Invalid rotation quaternion");
    Ok(())
}
