# Sown documentation site

Astro Starlight with strict TypeScript configuration and Tailwind CSS v4.

Tailwind is integrated through its Vite plugin and Starlight's compatibility
styles. Add shared styles and Tailwind theme customizations in
`src/styles/global.css`.

From this directory:

```sh
bun install --frozen-lockfile
bun run dev
```

The development server runs at `http://localhost:4321/sown/`.

Edit the manual in `src/content/docs`. Keep each page focused on one task or
reference topic, with a `title` in YAML frontmatter. Pages within each section
appear automatically in the sidebar; use `sidebar.order` to set their order.
Starlight supplies each page's main heading.

Write for players, world creators, server operators, and developers using Sown.
Describe concrete behaviour in present tense. Keep implementation history,
roadmaps, and unresolved design alternatives out of the manual. The repository's
`records` directory contains reference notes, separate from the published pages.

```sh
bun run check
bun run build
bun run preview
```

The static site is generated in `dist/`.

## GitHub Pages

In the repository settings, open **Pages** and select **GitHub Actions** as the
build and deployment source. The `Deploy documentation` workflow builds this
directory with Bun and publishes it at `https://floffah.github.io/sown/` when
changes to `site/` or the workflow reach `main`. You can also run it manually
from the Actions tab.

Astro's `site` and `base` settings define the published URL. Use relative links
between manual pages so they resolve beneath `/sown/`.
