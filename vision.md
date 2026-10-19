# Sown Project Vision
**Status:** Draft · October 2026

## 1. Introduction

Sown is an open-source platform for creating, hosting, exploring, and connecting interactive virtual worlds.

It envisions an interconnected universe of experiences, where players can explore vast environments, participate in games, socialise, create, and travel between worlds operated by independent communities and individuals.

Unlike conventional virtual world platforms, Sown is not intended to be a centralised service. It is an open ecosystem built around interoperable clients, servers, and world formats.

Anyone should be able to create a world, host a server, develop a compatible client, or contribute to the underlying technology without requiring permission from a central authority.

Sown aims to bring the openness of the web to interactive virtual environments.

## 2. The Vision

Sown imagines a virtual universe that feels interconnected while remaining fundamentally decentralised.

Players enter through their own personal spaces or shared public environments. From there, they can discover and travel to countless independently operated worlds.

These worlds might be games, cities, social environments, artistic experiences, educational spaces, or entirely new forms of interactive media.

A world might be operated by an individual on their personal computer, a small community on a dedicated server, or an organisation supporting thousands of simultaneous players.

Despite their independence, these worlds should feel like parts of a broader ecosystem.

**Sown should feel like one universe without requiring one organisation to own it.**

This is the central ambition of the project.

## 3. Guiding Principles

### 3.1 Open by design

Sown's core technology will be open-source.

Its protocols and formats should be openly documented, allowing independent implementations without reliance on proprietary infrastructure.

The project should encourage experimentation, modification, and community contributions.

The ability to create and operate a compatible Sown world must not depend on obtaining permission from the project's maintainers.

### 3.2 Decentralisation

Sown must not depend on a single organisation to host its worlds or operate its fundamental infrastructure.

Anyone should be able to host a server independently, much as individuals can host their own Minecraft servers or websites.

Official services may exist to make Sown more accessible, but they should provide convenience rather than exclusive capabilities.

The failure or disappearance of an official service should not prevent independently operated worlds from functioning.

### 3.3 Transparency and inspectability

Every Sown world should be inspectable by the people who visit it.

Worlds will be distributed through accessible source repositories containing the code, assets, and definitions necessary to construct their client-side experience.

Sown's client will provide browser-inspired developer tools, allowing players to explore the structure and behaviour of the worlds they visit.

Players should be able to inspect scenes, examine assets, read source code, observe network activity, and understand how an experience operates.

Transparency is intended to encourage trust, learning, experimentation, and accountability.

It is not, by itself, a security guarantee.

### 3.4 Security through isolation

Sown must assume that independently operated worlds are untrusted.

The client is responsible for safely loading and executing world content through a restricted runtime.

Worlds must not receive unrestricted access to the player's computer, personal files, identity, or other worlds' private data.

Sensitive capabilities should be explicitly controlled by the client.

Security should be enforced through technical boundaries rather than relying on the reputation or goodwill of world creators.

**The ability to inspect a world builds transparency. The sandbox provides protection.**

### 3.5 Player ownership and portability

Players should retain meaningful control over their identities, personal spaces, and portable data.

Sown intends to support federated identity infrastructure, allowing players to use hosted identity providers or operate their own.

A player's identity should not be permanently tied to one world, server, or service provider.

Where practical, personal data should be portable between providers.

Worlds may maintain their own authoritative gameplay data, but the broader ecosystem should avoid unnecessarily locking players into individual services.

### 3.6 Creative freedom

Sown should enable creators to build substantially different kinds of interactive experiences.

It should not prescribe a single genre, visual style, gameplay structure, or social model.

A creator might build a small puzzle game, a multiplayer racing experience, an expansive role-playing world, or a peaceful environment designed entirely for exploration.

The shared technology should provide the foundations necessary for interoperability while leaving creators free to define their experiences.

### 3.7 Accessibility of participation

Creating and hosting a world should be approachable without requiring specialised infrastructure.

Players should be able to explore Sown without operating their own servers.

Creators should be able to begin with simple local projects and progressively adopt more sophisticated capabilities.

Self-hosting should remain an option rather than an obligation.

The project should aim to make participation accessible to individuals, hobbyists, and communities, not exclusively large organisations.

## 4. A Universe of Independent Worlds

A Sown world is an independently defined interactive environment.

Worlds may contain scenes, objects, assets, scripts, and other resources that determine their appearance and behaviour.

A world may operate locally, through a player-hosted session, or on a dedicated server.

Servers provide the live functionality required for multiplayer interaction and authoritative simulation.

Players connect to independently hosted worlds through the Sown client.

Worlds should be addressable using a consistent mechanism, allowing players to connect directly or travel between experiences.

For example:

`sown://luminous.floffah.dev`

The precise addressing and discovery mechanisms will be established as the protocol develops.

### 4.1 Interconnected experiences

Worlds should be able to reference and provide access to other worlds.

A creator might place a portal inside their environment that transports players to an entirely different experience operated by another server.

These transitions should preserve appropriate aspects of the player's identity and experience, subject to the destination world's rules.

This creates the possibility of interconnected communities, collections of worlds, and larger virtual environments without requiring shared ownership or infrastructure.

### 4.2 The Capital City

Sown envisions a shared public environment known provisionally as the Capital City.

The Capital City serves as a welcoming entry point into the wider ecosystem.

It may provide public spaces for social interaction, opportunities for discovery, and connections to independently operated worlds.

Although it may be maintained by the Sown project, the Capital City should operate using the same fundamental technology available to other world creators.

It must not possess exclusive authority over the ecosystem.

Communities should be free to establish their own public environments and alternative entry points.

### 4.3 Personal homes

Every player should have the opportunity to maintain a personal space.

A personal home is a customisable environment that belongs to the player and can exist locally on their device.

It should not require a permanent internet connection or dedicated hosting service for ordinary local use.

Players may eventually be able to invite others into their homes through temporary hosting, peer-to-peer connectivity, or dedicated servers.

Personal homes provide a sense of continuity and ownership while remaining independent of the public worlds players visit.

## 5. Worlds as Source Code

Sown draws inspiration from the transparency and distribution model of the web.

Rather than treating a world as an opaque application downloaded from a central marketplace, Sown intends to make its source available to the client.

Worlds will be associated with Git repositories containing their definitions, assets, and source code.

When connecting to a server, the client can retrieve the source revision associated with that world and construct the experience using Sown's supported tooling.

This approach enables players to inspect the content they execute and gives developers familiar tools for version control, collaboration, and distribution.

Source repositories should not be restricted to a particular hosting provider.

The project should support independently hosted repositories and avoid requiring GitHub or another commercial service.

### 5.1 Developer tools

Sown intends to provide an integrated inspection environment inspired by web browser developer tools.

These tools should eventually support examination of:

- Scene hierarchies and object properties.
- Source code and scripts.
- Models, textures, audio, and other assets.
- Network activity and client-server communication.
- Runtime behaviour and performance.
- World permissions and capabilities.

Where appropriate, players may also be able to experiment with local modifications without changing the authoritative state of the world.

Inspectability should be considered a fundamental capability of the client rather than an optional developer feature.

## 6. Identity and Federation

Sown intends to support a federated identity system that allows players to authenticate across independently operated worlds.

Players should be able to use a convenient hosted identity provider without being required to operate infrastructure themselves.

Advanced users should have the option of self-hosting compatible identity services.

The identity system should distinguish between a player's persistent identity, their chosen provider, and their current game session.

Worlds should be able to verify player identities without receiving their account credentials.

Sown may also provide mechanisms for portable player information, such as profiles, avatar configurations, preferences, and certain forms of game data.

Access to this information should be governed by appropriate permissions.

The identity and data protocols will be designed specifically for Sown's requirements, taking inspiration from existing federated systems without necessarily adopting their complete architectures.

Federation is a long-term architectural goal, not a prerequisite for the earliest engine prototypes.

## 7. The Sown Client

The Sown client is the player's interface to the virtual universe.

It is responsible for retrieving and interpreting world content, rendering environments, handling user interaction, enforcing security boundaries, and communicating with servers.

Conceptually, the client serves a role similar to a web browser, but for interactive virtual worlds.

It should provide a consistent foundation across experiences while allowing worlds considerable freedom in their presentation and behaviour.

The client should also support local worlds, enabling players to explore and develop environments without requiring an external server.

Sown aims to define its protocols and formats clearly enough that alternative compatible clients may eventually be developed independently.

## 8. The Sown Server

A Sown server hosts the live simulation of a world.

It may be operated by an individual, community, organisation, or hosting provider.

Servers are responsible for authoritative gameplay behaviour, multiplayer state, and other functionality that cannot safely or reliably be delegated to clients.

A server should be able to operate without a graphical interface, making dedicated hosting practical.

Server operators should retain control over their worlds, including their rules, access policies, and persistence mechanisms.

Sown should not require server operators to register their worlds with an official directory or central platform.

## 9. Development Philosophy

Sown is an ambitious, long-term project.

Its development will follow an incremental approach, beginning with the smallest functional representation of a world and expanding towards the complete ecosystem.

Early development will focus on proving the core client and server architecture.

Initial prototypes may use hardcoded world representations, simple rendering, local file loading, and temporary player identities.

More sophisticated capabilities, including repository distribution, developer tools, interconnected worlds, personal homes, and federation, will be introduced progressively.

The project should favour working implementations over premature specification.

Architectural decisions should be documented as they emerge, with experimental designs remaining open to revision.

The goal is not to construct the entire universe immediately, but to establish a foundation upon which it can grow.

## 10. What Sown Is Not

Sown is not intended to be a proprietary virtual world service.

It is not intended to require creators to publish their work through a central marketplace.

It is not intended to make the project's maintainers the exclusive operators of identity, hosting, or world discovery.

It is not intended to impose a single visual identity, genre, or gameplay structure upon creators.

It is also not intended to guarantee that every independently operated world is trustworthy simply because its source code is accessible.

Sown provides the infrastructure for independent worlds to exist and interoperate. Responsibility for individual experiences remains with their operators, subject to the technical boundaries enforced by the platform.

## 11. Long-Term Ambition

Sown's ultimate ambition is to establish an open ecosystem of interconnected virtual environments.

A player might begin in their personal home, travel to a shared public city, meet friends, discover a new experience, and enter a world operated by somebody on the other side of the planet.

Creators might collaborate on enormous shared environments or develop small, independent experiences.

Communities might establish their own networks of worlds, identities, and services.

Developers might create alternative clients, servers, development tools, and infrastructure.

These activities should remain possible without depending on the continued existence of any single company or central platform.

Sown should be a foundation for virtual worlds rather than the owner of them.

---

### The central idea

**One universe. Countless worlds. No single owner.**

Sown aims to make interactive virtual worlds as open, inspectable, and independently operable as websites, while providing the shared foundations necessary for people to explore and connect them.

The universe belongs to the people who create and inhabit it.
