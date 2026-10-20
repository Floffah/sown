---
title: Inspecting worlds and permissions
sidebar:
  order: 4
---

Sown worlds are source projects. When you enter a world, the client retrieves its repository revision so you can inspect the client content.

Use developer tools to inspect scene hierarchies, object properties, source files, assets, network activity, permissions, and performance information. You can examine how the world is assembled and how it behaves, but local experiments have no authority over the server's simulation.

## Open developer tools

Press **F12** in the local client to show or hide the dark developer-tools panel
on the right side of the game view. The panel overlays the scene, which continues
rendering underneath it. Drag its left edge to adjust the width. Select **Network**
or **ECS** in the tab bar; the selected tab stays selected when you hide and
reopen the panel.

## Inspect network messages

Select **Network** to see the Sown messages exchanged by the client. Capture
starts when the connection opens and continues while developer tools are hidden,
so the list includes the handshake and initial world snapshots.

The upper pane lists messages in capture order. A green **→** badge marks a
received message; a red **←** badge marks a sent message. Each row shows its
message type and a truncated JSON preview. Hover over a row for its timestamp,
connection identifier, channel, and encoded size, including the stream length
prefix. Click a row to read and select its full formatted JSON in the lower pane.
Drag the horizontal divider to adjust the space given to each pane.

JSON follows the protobuf JSON mapping: default-valued fields can be omitted,
and 64-bit integers appear as strings. Malformed message bodies show a capture
error in the detail view. The inspector displays application messages, including
stream headers; QUIC acknowledgements and retransmissions are transport traffic
and do not appear here. Sent messages have been accepted by the local transport;
this does not confirm that the server received them.

History retains up to 1,000 messages and 4 MiB of message bodies and JSON text,
evicting the oldest entries when either limit is reached. The **dropped** count
reports captures lost when the capture queue fills. Capturing does not block
networking or interrupt gameplay when the queue is full. History is kept in
memory for the current client run.

## Source and reuse

You can inspect a project's source, but its licence determines whether you can copy, modify, or redistribute its code and assets. Check the licence before reusing them.

An advertised source revision identifies the client content. It does not prove that the server runs the same code or that its operator behaves honestly.

## Isolation

World content runs inside the client's restricted runtime. The client controls its access to local files, operating system capabilities, network resources, personal information, and other worlds' private data through explicit capabilities.

Repository content has no permission to execute host shell commands, package installation hooks, or unrestricted build scripts. You can read the source to understand the content, and the runtime restricts what that content can do.
