use std::collections::BTreeMap;

use anyhow::{Context, Result};
use engine::ecs::{
    Ecs,
    bevy_ecs::prelude::Entity,
    player::spawn_player,
    replication::{PeerOwner, Replicated},
    scene::Transform,
};
use protocol::v1::{
    EntitySnapshot, EventEnvelope, EventServerEntityCreated, EventServerEntityRemoved,
    EventServerPeerAdmitted, EventServerPeerRemoved, PeerSnapshot, Quaternion,
    ServerEntitiesSnapshot, ServerPeersSnapshot, Vector3, event_envelope::Payload,
};
use quinn::Connection;
use tokio::sync::mpsc;

const EVENT_QUEUE_CAPACITY: usize = 64;

pub(super) struct World {
    pub ecs: Ecs,
    pub tick: u64,
    next_peer_id: u32,
    next_entity_id: u64,
    peers: BTreeMap<u32, Peer>,
}

struct Peer {
    snapshot: PeerSnapshot,
    entity: Entity,
    connection: Connection,
    events: mpsc::Sender<EventEnvelope>,
}

impl World {
    pub fn new(ecs: Ecs) -> Self {
        Self {
            ecs,
            tick: 0,
            next_peer_id: 1,
            next_entity_id: 1,
            peers: BTreeMap::new(),
        }
    }

    pub fn allocate_peer_id(&mut self) -> Result<u32> {
        let id = self.next_peer_id;
        self.next_peer_id = id.checked_add(1).context("Peer IDs exhausted")?;
        Ok(id)
    }

    pub fn admit(
        &mut self,
        peer_id: u32,
        username: Option<String>,
        connection: Connection,
    ) -> Result<(u64, mpsc::Receiver<EventEnvelope>)> {
        anyhow::ensure!(!self.peers.contains_key(&peer_id), "Peer already admitted");
        let entity_id = self.next_entity_id;
        self.next_entity_id = entity_id.checked_add(1).context("Entity IDs exhausted")?;
        let entity = spawn_player(&mut self.ecs.world);
        self.ecs
            .world
            .entity_mut(entity)
            .insert((Replicated { entity_id }, PeerOwner(peer_id)));
        let snapshot = PeerSnapshot { peer_id, username };
        let entity_snapshot = self.entity_snapshot(entity);
        self.broadcast(&Payload::PeerAdmitted(EventServerPeerAdmitted {
            snapshot: Some(snapshot.clone()),
        }));
        self.broadcast(&Payload::EntityCreated(EventServerEntityCreated {
            snapshot: Some(entity_snapshot),
        }));
        let (events, receiver) = mpsc::channel(EVENT_QUEUE_CAPACITY);
        self.peers.insert(
            peer_id,
            Peer {
                snapshot,
                entity,
                connection,
                events: events.clone(),
            },
        );
        // Queue the baseline before exposing this peer to subsequent lifecycle events.
        events.try_send(EventEnvelope {
            payload: Some(Payload::InitialPeersSnapshot(ServerPeersSnapshot {
                server_tick: self.tick,
                snapshot: self
                    .peers
                    .values()
                    .map(|peer| peer.snapshot.clone())
                    .collect(),
            })),
        })?;
        events.try_send(EventEnvelope {
            payload: Some(Payload::InitialEntitiesSnapshot(ServerEntitiesSnapshot {
                server_tick: self.tick,
                snapshot: self
                    .peers
                    .values()
                    .map(|peer| self.entity_snapshot(peer.entity))
                    .collect(),
            })),
        })?;
        Ok((entity_id, receiver))
    }

    pub fn remove(&mut self, peer_id: u32) {
        let Some(peer) = self.peers.remove(&peer_id) else {
            return;
        };
        let entity_id = self
            .ecs
            .world
            .get::<Replicated>(peer.entity)
            .unwrap()
            .entity_id;
        self.ecs.world.despawn(peer.entity);
        self.broadcast(&Payload::EntityRemoved(EventServerEntityRemoved {
            entity_id,
        }));
        self.broadcast(&Payload::PeerRemoved(EventServerPeerRemoved { peer_id }));
    }

    fn entity_snapshot(&self, entity: Entity) -> EntitySnapshot {
        let transform = self.ecs.world.get::<Transform>(entity).unwrap();
        let [x, y, z] = transform.translation;
        let [qx, qy, qz, qw] = transform.rotation;
        let [sx, sy, sz] = transform.scale;
        EntitySnapshot {
            entity_id: self.ecs.world.get::<Replicated>(entity).unwrap().entity_id,
            owner_peer_id: self.ecs.world.get::<PeerOwner>(entity).unwrap().0,
            transform: Some(protocol::v1::Transform {
                translation: Some(Vector3 { x, y, z }),
                rotation: Some(Quaternion {
                    x: qx,
                    y: qy,
                    z: qz,
                    w: qw,
                }),
                scale: Some(Vector3 {
                    x: sx,
                    y: sy,
                    z: sz,
                }),
            }),
        }
    }

    fn broadcast(&self, payload: &Payload) {
        for peer in self.peers.values() {
            if peer
                .events
                .try_send(EventEnvelope {
                    payload: Some(payload.clone()),
                })
                .is_err()
            {
                // A dropped reliable event would leave this peer with a different world.
                peer.connection
                    .close(1_u8.into(), b"Events receiver too slow");
            }
        }
    }
}
