use anyhow::{Context, Result, bail, ensure};
use protocol::v1::{
    ControlClientJoin, ControlEnvelope, EventEnvelope, ServerEntitiesSnapshot, ServerPeersSnapshot,
    control_envelope, event_envelope,
};

use super::{ClientStreams, read_message, write_control};
use crate::session::JoinedWorld;

/// Requests admission, reads Joined, and validates both reliable baseline snapshots.
/// Call this after the local world is ready. Keep the stream handles for Play.
///
/// # Errors
/// Returns an error for rejected admission, invalid baselines, or stream failures.
/// The caller supplies a deadline and closes the connection on failure.
pub async fn join(streams: &mut ClientStreams) -> Result<JoinedWorld> {
    let request = control_envelope::Payload::ClientJoin(ControlClientJoin {});
    write_control(&mut streams.control_send, request).await?;

    let response: ControlEnvelope = read_message(&mut streams.control_recv).await?;
    let player_entity_id = match response.payload {
        Some(control_envelope::Payload::ServerJoined(joined)) => joined.player_entity_id,
        Some(control_envelope::Payload::ServerError(error)) => {
            bail!("Server rejected join: {}", error.message)
        }
        _ => bail!("Expected ServerJoined on Control"),
    };
    ensure!(player_entity_id != 0, "Invalid player entity ID in Joined");

    let first = read_message(&mut streams.events_recv).await?;
    let second = read_message(&mut streams.events_recv).await?;
    let (peers, entities) = parse_initial_snapshots([first, second])?;

    let world = JoinedWorld {
        peer_id: streams.peer_id,
        player_entity_id,
        server_tick: peers.server_tick,
        peers: peers.snapshot,
        entities: entities.snapshot,
    };
    world.validate()?;
    Ok(world)
}

fn parse_initial_snapshots(
    messages: [EventEnvelope; 2],
) -> Result<(ServerPeersSnapshot, ServerEntitiesSnapshot)> {
    let mut peers = None;
    let mut entities = None;

    for message in messages {
        match message.payload {
            Some(event_envelope::Payload::InitialPeersSnapshot(snapshot)) => peers = Some(snapshot),
            Some(event_envelope::Payload::InitialEntitiesSnapshot(snapshot)) => {
                entities = Some(snapshot);
            }
            _ => bail!("Expected both initial snapshots on Events"),
        }
    }

    let peers = peers.context("Expected both initial snapshots on Events")?;
    let entities = entities.context("Expected both initial snapshots on Events")?;
    ensure!(
        peers.server_tick == entities.server_tick,
        "Initial snapshot ticks differ"
    );
    Ok((peers, entities))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_snapshots_accept_either_order_and_require_a_matching_pair() -> Result<()> {
        let peers = EventEnvelope {
            payload: Some(event_envelope::Payload::InitialPeersSnapshot(
                ServerPeersSnapshot {
                    server_tick: 7,
                    snapshot: vec![],
                },
            )),
        };
        let entities = EventEnvelope {
            payload: Some(event_envelope::Payload::InitialEntitiesSnapshot(
                ServerEntitiesSnapshot {
                    server_tick: 7,
                    snapshot: vec![],
                },
            )),
        };
        let forward = parse_initial_snapshots([peers.clone(), entities.clone()])?;
        let reversed = parse_initial_snapshots([entities.clone(), peers.clone()])?;
        assert_eq!(forward, reversed);

        for messages in [
            [peers.clone(), peers.clone()],
            [entities.clone(), entities],
            [peers.clone(), EventEnvelope { payload: None }],
            [
                peers,
                EventEnvelope {
                    payload: Some(event_envelope::Payload::InitialEntitiesSnapshot(
                        ServerEntitiesSnapshot {
                            server_tick: 8,
                            snapshot: vec![],
                        },
                    )),
                },
            ],
        ] {
            assert!(parse_initial_snapshots(messages).is_err());
        }
        Ok(())
    }
}
