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

We maintain a Astro Starlight based documentation site which is aimed at both non-technical users of the platform AND developers who are looking to build worlds but NOT developers working on Sown itself (this will be documented in a future standards-driven manual).

If you make a change that contradicts this documentation, you may change it but with permission from the user first and foremost. If instructed to make a change to Sown that conflicts with the documentation OR is undocumented, you may continue working on the changes but notify the user that you will update the docs and then do so to ensure documentation consistency. 

The docs you must update are primarily code comments (based on the above rules) and the Astro Starlight documentation site. But, if a private docs or records folder exists (such as `records/`) then this follows the same rules.

When updating documentation you MUST follow the following rules:
- The documentation should not explain the protocol in such a detail that the user does not care about, BUT should explain the protocol where it makes sense for contextual reasons. Assume readers dont understand the inner workings of sown, and frankly dont care about understanding it. THe only exception to this is the Wire and Identity documentation sections, which are only in the site while there is no true pdf manual. Sections like world are a good example of what should be there, because developers using sown (but not working on sown) must understand the world format. Same as system admins must understand how to run a server. Etc
- IMPORTANT! - the documentation MUST be detailed, contextual, and DEFINITIVE. Abstract or conceptual documentation will be rejected. Documentation should follow an information structure and include continuity where possible. Largely it should be logically coherent
- Documentation should follow the grammar and sentence structure rules defined below. You should also write documentation in conjunction with ANY humanisation skills or other language writing skills that the user has installed locally or globally (e.g. humanizer, avoid-ai-writing, etc)
- At this current prototype stage the documentation is intentionally not framed as "this is what it does now, here's what it will do in the future". The entire project proposal is taken as if it already exists and works, hence why far more systems are documented in far more detail than exists today. Bring this forward. If you are instructed to document something which includes writing about systems that don't exist, pretend they exist and ask the user for clarification where necessary.

When making updates to Sown, the order of design precedence is as follows:
1. What the user explicitly tells you (but if other documentation sources conflict, challenge the user immediately and make no changes)
2. Ignored or personal developer documentation files (such as in `records/`)
3. docs in `protocol/`
4. The documentation site in `site/content/docs`
5. `/vision.md`

Importantly, as it stands: the wire and identity sections of the docs site are completely provisional and draft, conflicts here do not matter as much and shouldn't require you to stop. You must still stop for all other documentation sections and sources.

If the changes you are making agree with documentation (such as the protocol) higher in priority in the precedence list above but disagrees with later documentation, don't consider this a conflict but rather notify the user that the later documentation needs updating.

## Rust file organization

Prefer this order within Rust modules: imports and module declarations, constants,
types and their implementations (including methods), free functions, then tests.

Keep closely coupled definitions adjacent, including a type's implementations or
dedicated helpers, even when another type comes afterwards. This adjacency takes
precedence over the default order. Preserve coherent groups and existing module
boundaries; do not split or add abstractions merely to enforce ordering.

## Sentence construction and grammatical style
Write direct, natural technical prose that describes the actual entities, actions, and properties involved.

Avoid repeatedly using generic singular subjects such as "a server", "a user", "a world", or "a Personal Data Store" when referring to general behaviour. Prefer natural plurals, established definite references, or direct second-person constructions depending on context.

Avoid unnecessarily assigning agency to abstract concepts. Systems, protocols, destinations, and processes should not routinely "decide", "know", "want", or "choose" unless those verbs accurately describe their behaviour.

Prefer concrete subjects over nominalised activities. For example, write "The application works offline" rather than "Local use works offline".

When describing permanent architectural properties, state those properties directly. Prefer "Personal Data Stores don't have authority over world data" over unnecessarily action-oriented descriptions such as "A Personal Data Store does not grant authority over world data", provided the meaning is preserved.

Avoid defaulting to impersonal constructions such as "X enables Y", "X allows Y", or "X provides a mechanism for Y". Where appropriate, explain what the reader can do instead.

Use pronouns and established references naturally. Don't reintroduce an entity by its full name in every sentence.
Preserve technical precision. Do not replace specific architectural terms with broader, vaguer language merely to make sentences sound conversational.

Vary sentence construction according to meaning and context, not for the sake of variety. These are preferences, not absolute grammatical prohibitions.

Generally, prefer definitive contextual writing, and avoid abstract and conceptual information wherever possible.
