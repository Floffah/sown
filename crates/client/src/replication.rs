use std::collections::{BTreeMap, HashMap};

use anyhow::{Context, Result, bail, ensure};
use engine::ecs::{
    bevy_ecs::prelude::{Entity, World},
    replication::{PeerOwner, Replicated},
    scene::Transform,
};
use networking::session::{JoinedWorld, validate_entity};
use protocol::v1::{EntitySnapshot, EventEnvelope, PeerSnapshot, event_envelope::Payload};
use quinn::Connection;

pub(super) enum Update {
    Joined {
        world: JoinedWorld,
        connection: Connection,
    },
    Event(EventEnvelope),
}

pub(super) struct Replication {
    local_player: Entity,
    local_player_id: Option<u64>,
    connection: Option<Connection>,
    peers: BTreeMap<u32, PeerSnapshot>,
    entities: HashMap<u64, Entity>,
}

impl Replication {
    pub fn new(local_player: Entity) -> Self {
        Self {
            local_player,
            local_player_id: None,
            connection: None,
            peers: BTreeMap::new(),
            entities: HashMap::new(),
        }
    }

    pub fn apply(&mut self, ecs: &mut World, update: Update) -> Result<()> {
        match update {
            Update::Joined { world, connection } => {
                self.clear(ecs);
                self.connection = Some(connection);
                self.apply_baseline(ecs, world)?;
            }
            Update::Event(event) => match event.payload {
                Some(Payload::PeerAdmitted(peer)) => {
                    let peer = peer.snapshot.context("Missing peer snapshot")?;
                    ensure!(
                        peer.peer_id != 0 && !self.peers.contains_key(&peer.peer_id),
                        "Invalid peer admission"
                    );
                    self.peers.insert(peer.peer_id, peer);
                }
                Some(Payload::EntityCreated(entity)) => {
                    self.create(ecs, &entity.snapshot.context("Missing entity snapshot")?)?;
                }
                Some(Payload::EntityRemoved(entity)) => {
                    ensure!(
                        Some(entity.entity_id) != self.local_player_id,
                        "Server removed the local player"
                    );
                    let local = self
                        .entities
                        .remove(&entity.entity_id)
                        .context("Unknown removed entity")?;
                    ecs.despawn(local);
                }
                Some(Payload::PeerRemoved(peer)) => {
                    ensure!(
                        !self
                            .entities
                            .values()
                            .any(|entity| ecs.get::<PeerOwner>(*entity)
                                == Some(&PeerOwner(peer.peer_id))),
                        "Peer removed before its entities"
                    );
                    ensure!(
                        self.peers.remove(&peer.peer_id).is_some(),
                        "Unknown removed peer"
                    );
                }
                _ => bail!("Unexpected Events message during Play"),
            },
        }
        Ok(())
    }

    fn apply_baseline(&mut self, ecs: &mut World, world: JoinedWorld) -> Result<()> {
        world.validate()?;
        self.local_player_id = Some(world.player_entity_id);
        self.peers = world
            .peers
            .into_iter()
            .map(|peer| (peer.peer_id, peer))
            .collect();
        for entity in &world.entities {
            self.create(ecs, entity)?;
        }
        println!(
            "Entered Play: peer {}, player entity {}, baseline tick {}",
            world.peer_id, world.player_entity_id, world.server_tick
        );
        Ok(())
    }

    pub fn reject(&mut self, ecs: &mut World) {
        if let Some(connection) = self.connection.take() {
            connection.close(1_u8.into(), b"Invalid server state");
        }
        self.clear(ecs);
    }

    fn create(&mut self, ecs: &mut World, snapshot: &EntitySnapshot) -> Result<()> {
        validate_entity(snapshot)?;
        ensure!(
            !self.entities.contains_key(&snapshot.entity_id),
            "Duplicate entity creation"
        );
        ensure!(
            self.peers.contains_key(&snapshot.owner_peer_id),
            "Unknown entity owner"
        );
        let transform = snapshot.transform.unwrap();
        let translation = transform.translation.unwrap();
        let rotation = transform.rotation.unwrap();
        let scale = transform.scale.unwrap();
        let local = if Some(snapshot.entity_id) == self.local_player_id {
            self.local_player
        } else {
            ecs.spawn_empty().id()
        };
        ecs.entity_mut(local).insert((
            Replicated {
                entity_id: snapshot.entity_id,
            },
            PeerOwner(snapshot.owner_peer_id),
            Transform {
                translation: [translation.x, translation.y, translation.z],
                rotation: [rotation.x, rotation.y, rotation.z, rotation.w],
                scale: [scale.x, scale.y, scale.z],
            },
        ));
        self.entities.insert(snapshot.entity_id, local);
        Ok(())
    }

    pub fn clear(&mut self, ecs: &mut World) {
        for (_, entity) in self.entities.drain() {
            if entity == self.local_player {
                ecs.entity_mut(entity).remove::<(Replicated, PeerOwner)>();
            } else {
                ecs.despawn(entity);
            }
        }
        self.local_player_id = None;
        self.connection = None;
        self.peers.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::ecs::player::{LocalPlayer, spawn_local_player};
    use protocol::v1::{EventServerEntityCreated, EventServerEntityRemoved, Quaternion, Vector3};

    fn entity(entity_id: u64) -> EntitySnapshot {
        EntitySnapshot {
            entity_id,
            owner_peer_id: 12,
            transform: Some(protocol::v1::Transform {
                translation: Some(Vector3 {
                    x: 8.0,
                    y: 2.0,
                    z: 1.0,
                }),
                rotation: Some(Quaternion {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 1.0,
                }),
                scale: Some(Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                }),
            }),
        }
    }

    fn baseline() -> JoinedWorld {
        JoinedWorld {
            peer_id: 12,
            player_entity_id: 72,
            server_tick: 9,
            peers: vec![PeerSnapshot {
                peer_id: 12,
                username: None,
            }],
            entities: vec![entity(71), entity(72)],
        }
    }

    #[test]
    fn correlates_assigned_player_not_just_ownership_and_cleans_up() -> Result<()> {
        let mut world = World::new();
        let local = spawn_local_player(&mut world);
        let mut replication = Replication::new(local);
        replication.apply_baseline(&mut world, baseline())?;
        assert_eq!(replication.entities[&72], local);
        let remote = replication.entities[&71];
        assert_ne!(remote, local);
        assert!(world.get::<LocalPlayer>(remote).is_none());
        assert_eq!(
            world.get::<Transform>(local).unwrap().translation,
            [8.0, 2.0, 1.0]
        );
        replication.apply(
            &mut world,
            Update::Event(EventEnvelope {
                payload: Some(Payload::EntityCreated(EventServerEntityCreated {
                    snapshot: Some(entity(73)),
                })),
            }),
        )?;
        let created = replication.entities[&73];
        replication.apply(
            &mut world,
            Update::Event(EventEnvelope {
                payload: Some(Payload::EntityRemoved(EventServerEntityRemoved {
                    entity_id: 73,
                })),
            }),
        )?;
        assert!(world.get_entity(created).is_err());
        replication.clear(&mut world);
        assert!(world.get_entity(remote).is_err());
        assert!(world.get::<LocalPlayer>(local).is_some());
        assert!(world.get::<Replicated>(local).is_none());
        assert!(replication.entities.is_empty());
        Ok(())
    }

    #[test]
    fn rejects_invalid_baselines_before_mutating_ecs() {
        let mut world = World::new();
        let local = spawn_local_player(&mut world);
        let mut replication = Replication::new(local);
        for invalid in 0..5 {
            let mut baseline = baseline();
            match invalid {
                0 => baseline.player_entity_id = 999,
                1 => baseline.entities.push(entity(71)),
                2 => baseline.entities[0].owner_peer_id = 999,
                3 => baseline.entities[0].transform = None,
                _ => {
                    baseline.entities[0]
                        .transform
                        .as_mut()
                        .unwrap()
                        .translation
                        .as_mut()
                        .unwrap()
                        .x = f32::NAN;
                }
            }
            assert!(replication.apply_baseline(&mut world, baseline).is_err());
            assert!(replication.entities.is_empty());
            assert_eq!(
                world.get::<Transform>(local).unwrap().translation,
                [3.0, 3.0, 5.0]
            );
        }
    }
}
