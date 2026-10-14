When working on this application, consult the docs folder. Especially the files: vison.md.

When developing in this application, to check your work you must run checks, clippy, tests using `just check`

## Server persistent files

Place user/developer-editable configuration files in the process working
directory. Store managed cross-run state, including generated TLS credentials,
under `.sown` in that same working directory. Do not use the executable's
directory or a global user data directory for this state. Keep `.sown` out of Git.

## Code comments and documentation

Describe only what the specific code or subsystem does now. Omit reassurances
about missing future features, such as "session/network identity is not assigned
yet."

Use `TODO` comments only for concrete work that needs to be done soon within the
subsystem being documented. Broader capabilities and structural plans belong in
the roadmap, vision, or PRD, rather than code comments or TODOs.

If an agent needs to explain deferred work or reassure the user about scope, do
so in chat rather than in a file.

## Rust file organization

Prefer this order within Rust modules: imports and module declarations, constants,
types and their implementations (including methods), free functions, then tests.

Keep closely coupled definitions adjacent, including a type's implementations or
dedicated helpers, even when another type comes afterwards. This adjacency takes
precedence over the default order. Preserve coherent groups and existing module
boundaries; do not split or add abstractions merely to enforce ordering.
