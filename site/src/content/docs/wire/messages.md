---
title: Messages and snapshots
sidebar:
  order: 6
---

The message schema uses the Protocol Buffers package `sown.protocol.v1`. Each channel has separate envelopes for client-sent and server-sent messages:

| Channel | Client sends | Server sends |
| --- | --- | --- |
| Control | `ControlClientEnvelope` | `ControlServerEnvelope` |
| Events | `EventClientEnvelope` | `EventServerEnvelope` |
| Datagrams | `DatagramClientEnvelope` | `DatagramServerEnvelope` |

Send the directional envelope directly. There is no enclosing bidirectional
envelope. Receivers select the decoding type from the channel and the sender's
role, then validate the payload against the session phase. The initial
`ClientStreamHeader` remains a separate framed message.

## Common values

| Type | Fields |
| --- | --- |
| `Vector3` | `float x`, `y`, `z` |
| `Quaternion` | `float x`, `y`, `z`, `w` |
| `Transform` | `translation`, `rotation`, `scale` |
| `PeerSnapshot` | `uint32 peer_id`, optional `string username` |
| `EntitySnapshot` | `uint64 entity_id`, `uint32 owner_peer_id`, `Transform transform` |

Peers are network participants; entities are world objects. Peer snapshots describe participants' networking identity, and player entity snapshots describe their world objects.

## Control schema

`ControlClientEnvelope` defines `client_hello` (field `1`) and `client_join` (field `4`). `ControlServerEnvelope` defines `server_welcome` (field `2`), `server_error` (field `3`), and `server_joined` (field `5`). Hello carries the protocol version and optional username; Welcome carries protocol version and peer ID. `ControlClientJoin` has an empty body. `ControlServerJoined` carries the spawned player's `uint64 player_entity_id` in field `1`; its envelope payload uses field `5`. `ControlServerError` carries a message string in field `2`; field `1` and the name `error_id` are reserved.

Hello and Welcome belong to Handshake. After receiving Welcome and preparing
the local world, the client sends `ControlClientJoin` to start Join. The server
waits for the Join request and Events stream setup before admitting the peer.
After admission, `ControlServerJoined` identifies the client's player entity.
The client matches this ID against `entity_id` in snapshots and lifecycle events;
`owner_peer_id` alone does not identify which owned entity is its player.
The [session phases](../session/) define the order of these exchanges.

## Events schema

`EventServerEnvelope` carries the following payloads, using fields `1` through
`6` in table order. `EventClientEnvelope` has no fields or valid application
messages; sending one causes the server to reject the session.

| Payload | Content |
| --- | --- |
| `peer_admitted` | Peer snapshot |
| `peer_removed` | Peer ID |
| `entity_created` | Entity snapshot |
| `entity_removed` | Entity ID |
| `initial_peers_snapshot` | Server tick and repeated peer snapshots |
| `initial_entities_snapshot` | Server tick and repeated entity snapshots |

## Datagram schema

`DatagramClientEnvelope` defines `local_player_move` (field `1`). `DatagramServerEnvelope` defines `peers_snapshot` (field `2`) and `entities_snapshot` (field `3`). Local player movement contains a transform. Snapshot collections contain a `uint64 server_tick` and repeated snapshot entries.

The world server determines authoritative gameplay outcomes. Clients have no authority over a world's rules or stored progress through portable player data or reported transforms.
