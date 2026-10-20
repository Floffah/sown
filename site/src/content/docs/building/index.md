---
title: Building with Sown
sidebar:
  order: 1
---

Use Sown's Rust libraries to load world files, run simulation systems, and render the scene. You can also build a compatible implementation using the [open wire formats](/wire/) without the reference libraries.

- [Load and simulate worlds](/building/world-engine/) covers parsing, ECS ownership, systems, and movement input.
- [Cameras and rendering](/building/rendering/) covers scene components, camera requirements, and frame outcomes.
- [Networking helpers](/wire/rust/) covers buffered framing and Quinn integration.

World files describe the content you create. The engine holds its live state, and the client supplies input and elapsed time. The renderer reads scene and camera components; the application manages the window and input loop.
