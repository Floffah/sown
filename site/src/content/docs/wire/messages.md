---
title: Messages and snapshots
sidebar:
  order: 6
---

The message schema uses the Protocol Buffers package `sown.protocol.v1`. Reliable messages are wrapped in the envelope for their channel. Datagram payloads use `DatagramEnvelope`.

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

`ControlEnvelope` defines `client_hello`, `server_welcome`, `server_error`, `client_join`, and `server_joined` payloads. Hello carries the protocol version and optional username; Welcome carries protocol version and peer ID. `ControlClientJoin` has an empty body. `ControlServerJoined` carries the spawned player's `uint64 player_entity_id` in field `1`; its envelope payload uses field `5`. `ControlServerError` carries a message string in field `2`; field `1` and the name `error_id` are reserved.

Hello and Welcome belong to Handshake. After receiving Welcome and preparing
the local world, the client sends `ControlClientJoin` to start Join. The server
waits for the Join request and Events stream setup before admitting the peer.
After admission, `ControlServerJoined` identifies the client's player entity.
The client matches this ID against `entity_id` in snapshots and lifecycle events;
`owner_peer_id` alone does not identify which owned entity is its player.
The [session phases](../session/) define the order of these exchanges.

## Events schema

| Payload | Content |
| --- | --- |
| `peer_admitted` | Peer snapshot |
| `peer_removed` | Peer ID |
| `entity_created` | Entity snapshot |
| `entity_removed` | Entity ID |
| `initial_peers_snapshot` | Server tick and repeated peer snapshots |
| `initial_entities_snapshot` | Server tick and repeated entity snapshots |

## Datagram schema

`DatagramEnvelope` defines `local_player_move`, `peers_snapshot`, and `entities_snapshot`. Local player movement contains a transform. Snapshot collections contain a `uint64 server_tick` and repeated snapshot entries.

The world server determines authoritative gameplay outcomes. Clients have no authority over a world's rules or stored progress through portable player data or reported transforms.
