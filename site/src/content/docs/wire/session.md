---
title: Session establishment
sidebar:
  order: 5
---

A native session uses two client-opened, long-lived bidirectional streams on one QUIC connection.

## Session phases

Connections progress through Handshake, Join, Play, and Disconnecting. Clients
and servers track their own progress through these phases; network delivery
means they do not necessarily change phase at the same instant.

| Phase | Exchange and result |
| --- | --- |
| Handshake | Hello and Welcome establish protocol compatibility and assign the connection's peer ID. The client prepares its local world. |
| Join | `ControlClientJoin` starts admission to the world. The server spawns the peer's player, identifies it in Joined, and sends the initial world state. |
| Play | The client has applied its initial state and exchanges gameplay messages with the server. |
| Disconnecting | The connection closes and the server removes the departing peer and its player. |

## Handshake

1. Open the first bidirectional stream.
2. Send a framed `ClientStreamHeader` with `stream_id = 0` (Control).
3. Send a framed `ControlEnvelope` containing `ControlClientHello` with `protocol_version = 1`.
4. Read the server's `ControlEnvelope` containing `ControlServerWelcome` on the same stream.
5. Check that Welcome reports protocol version `1` and retain its assigned `peer_id`.

Welcome identifies the connection's peer. Player creation takes place during
Join. Once Welcome is received, the client can finish loading the world before
requesting admission.

The server applies a 10-second deadline from accepting the connection through
completing the Control handshake. The client applies a 10-second deadline to
the Hello/Welcome exchange and Events stream setup. World loading is separate from that handshake deadline. After sending Join, the
native client applies a separate 10-second deadline to receiving Joined and both
initial snapshots.

## Events stream

After validating Welcome, open a second bidirectional stream and send a framed `ClientStreamHeader` with `stream_id = 1` (Events). The server applies a 10-second header deadline after accepting this stream.

Keep both stream pairs alive for the connection. The receiver handles Events independently of idle Control. The native client uses QUIC keepalives every 5 seconds without sending application messages.

## Join

After loading the world and sending the Events header, send an empty
`ControlClientJoin` inside a `ControlEnvelope` on Control. This message starts
the Join phase.

The server waits for both the Join request and a valid Events header before
admitting the peer. Their arrival order can differ because they use separate
streams. Once both are received, the server creates the peer's player entity
and associates it with the peer ID assigned in Welcome. Other joined clients
receive the peer admission and entity creation events on Events.

The server sends `ControlServerJoined` on Control after spawning the player.
Its `player_entity_id` identifies that player's replicated entity. The client
retains this ID and matches it against `entity_id` in snapshots and subsequent
entity events. A peer can own several entities, so ownership alone does not
identify its player.

Joined confirms admission; the client remains in Join until its initial state
is ready. Control and Events are separate streams, so Joined and the initial
snapshots can arrive in either order.

The joining client receives `initial_peers_snapshot` and
`initial_entities_snapshot` on Events. Both describe the same `server_tick` and
include empty collections when there are no matching entries. The entity
snapshot includes the newly created player. The client applies both snapshots
to establish its baseline before entering Play. It also checks that the assigned
player exists in the entity snapshot and belongs to the peer ID from Welcome.

## Play

The server queues the initial snapshots before subsequent lifecycle events on
Events. The client processes those events in stream order. Subsequent snapshot
updates arrive as datagrams, using the same snapshot payloads.

The client applies datagram snapshots only after establishing its initial
baseline, and rejects snapshots older than the state already applied for that
snapshot collection. A datagram received before the baseline is complete does
not replace either initial reliable snapshot.

Peer IDs identify connections. Entity IDs identify replicated world objects;
`owner_peer_id` associates each object with its peer. The server retains final
authority over gameplay outcomes.

## Rejection and closure

The receiver rejects unexpected headers, unsupported versions, duplicate Control
channels, malformed frames, and empty Events envelopes. It also rejects Control
messages that are invalid for the connection's phase, including another Hello
after Welcome and repeated Join requests. Rejected connections close with
application error code `1` and a textual reason.

Disconnecting can begin during any phase, including when the transport fails.
The server removes an admitted peer's player and sends `entity_removed` and
`peer_removed` events to the remaining joined clients. Connections that close
before admission have no player entity to remove.

A normal server shutdown closes the endpoint with code `0` and waits for connection shutdown. Transport admission and persistent identity authentication are separate operations.
