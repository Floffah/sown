# Sown

> [!NOTE]
> Sown is not finished but I think it is a very interesting idea. If you want to test it its pretty simple: install [Rust](https://rust-lang.org/), install [Just](https://github.com/casey/just), clone the repo then in two terminals run `just run-server` and `just run-client`. If you wanna see what its doing under the hood, press F12 and my devtools like panel will open

Sown is an open-source platform for creating, hosting, and exploring interconnected virtual worlds. Play games, meet friends, build a place of your own, and travel between experiences run by independent creators and communities.

Sown brings the openness of the web to interactive virtual environments. Its client works like a browser for worlds: each world has its own address, content, and operator, while shared formats and protocols let them connect into a wider universe.

## Explore interconnected worlds

A Sown world can be a multiplayer game, a city, an art installation, a classroom, or a secluded place to explore. Creators choose the appearance, behaviour, and rules of their own worlds.

Connect directly to a world through its `sown://` address, or step through a portal to another experience. Worlds can link to one another across independently operated servers, letting communities build their own networks of places to visit.

The Capital City is a shared public hub for meeting people and discovering worlds. Communities can create their own hubs, and you can enter worlds directly from either the desktop or browser client.

## Make yourself at home

Your personal home is a customisable world that lives on your device. Use it offline, make it your own, or host it if you want to invite others.

Your identity travels with you between worlds. Choose a hosted identity provider or run your own, with control over which worlds can access your personal information. You can change providers while keeping your identity. Each world manages its own gameplay rules and progress, and decides which portable data it accepts.

## Transparent worlds and easy creation

Worlds are source projects containing scene definitions, scripts, models, textures, audio, and other assets. They live in Git repositories, giving creators familiar tools for version control and collaboration, with a choice of where to host their work.

Start with a local project, develop it on your own machine, and host it for others to visit. The same foundations support small personal spaces and shared multiplayer experiences.

Built-in developer tools let you inspect the worlds you visit: browse their source, explore scene structure, examine assets, and observe network activity and performance. You can learn how an experience works and experiment with local changes. Each project's licence determines how its content may be reused or redistributed.

World content runs in a sandbox, with limited access to your device and personal data. Source inspection helps you understand a world; runtime isolation enforces good boundaries.

## Host it on your own terms

Run a world from your own computer, a community server, or a hosting provider. Server operators control their world's rules, access, and persistent gameplay data.

Hosting a compatible world requires no approval from Sown's maintainers or registration with an official directory. Independent worlds continue operating independently of official services.

Sown's open formats and protocols also let developers build compatible clients, servers, and tools. The ecosystem belongs to the people.

## Learn more

Read the [project vision](vision.md) for the principles behind Sown and a fuller picture of its universe. Also an [indev docs site](site/src/content/docs)
