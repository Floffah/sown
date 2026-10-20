---
title: Cameras and rendering
sidebar:
  order: 3
---

The renderer draws entities with `Transform`, `Mesh`, and `Material` components. `Transform` contains translation, an X/Y/Z/W quaternion, and scale. `Material.color` contains RGB values. `Mesh::Primitive(MeshPrimitive::Cube)` selects built-in cube geometry.

Component additions, changes, and removals appear on the next redraw. An empty scene displays the background. Mesh batches share one render pass and one depth buffer.

## Active camera

Provide exactly one entity containing `Player`, `LocalPlayer`, `Camera`, and `Transform`. Other camera entities do not supply the active view.

The camera looks along local negative Z with positive Y as up. Its transform supplies position and orientation; scale is ignored. `Camera` contains vertical field of view in radians and positive near/far clipping distances.

`spawn_local_player` supplies a camera at `(3, 3, 5)`, oriented toward the origin, with a 60-degree vertical field of view and clipping distances of `0.1` and `100`. Invalid camera transforms or projection settings produce a rendering error.

## Application integration

The application owns the window or canvas, event loop, input, and redraw scheduling. Pass an owned render target and dimensions to `renderer::Renderer::new`, then call `resize` to configure the surface.

`render` reports a `RenderOutcome`:

| Outcome | Application response |
| --- | --- |
| `Presented` | Schedule the next redraw |
| `Retry` | Request another redraw |
| `Skipped` | Wait for a window event to resume drawing |

Supply current movement input and tick the ECS before drawing. The reference client caps elapsed frame time at 100 ms to limit movement jumps after a stall.
