---
title: Inspecting worlds and permissions
sidebar:
  order: 4
---

Sown worlds are source projects. When you enter a world, the client retrieves its repository revision so you can inspect the client content.

Use developer tools to inspect scene hierarchies, object properties, source files, assets, network activity, permissions, and performance information. You can examine how the world is assembled and how it behaves, but local experiments have no authority over the server's simulation.

## Source and reuse

You can inspect a project's source, but its licence determines whether you can copy, modify, or redistribute its code and assets. Check the licence before reusing them.

An advertised source revision identifies the client content. It does not prove that the server runs the same code or that its operator behaves honestly.

## Isolation

World content runs inside the client's restricted runtime. The client controls its access to local files, operating system capabilities, network resources, personal information, and other worlds' private data through explicit capabilities.

Repository content has no permission to execute host shell commands, package installation hooks, or unrestricted build scripts. You can read the source to understand the content, and the runtime restricts what that content can do.
