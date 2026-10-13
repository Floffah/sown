When working on this application, consult the docs folder. Especially the files: vison.md.

When developing in this application, to check your work you must run checks, clippy, tests using `just check`

## Rust file organization

Prefer this order within Rust modules: imports and module declarations, constants,
types and their implementations (including methods), free functions, then tests.

Keep closely coupled definitions adjacent, including a type's implementations or
dedicated helpers, even when another type comes afterwards. This adjacency takes
precedence over the default order. Preserve coherent groups and existing module
boundaries; do not split or add abstractions merely to enforce ordering.
