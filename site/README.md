# Sown documentation site

Astro Starlight with strict TypeScript configuration and Tailwind CSS v4.

Tailwind is integrated through its Vite plugin and Starlight's compatibility
styles. Add shared styles and Tailwind theme customizations in
`src/styles/global.css`.

From this directory:

```sh
npm ci
npm run dev
```

The development server runs at `http://localhost:4321`.

Edit the manual in `src/content/docs`. Keep each page focused on one task or
reference topic, with a `title` in YAML frontmatter. Pages within each section
appear automatically in the sidebar; use `sidebar.order` to set their order.
Starlight supplies each page's main heading.

Write for players, world creators, server operators, and developers using Sown.
Describe concrete behaviour in present tense. Keep implementation history,
roadmaps, and unresolved design alternatives out of the manual. The repository's
`records` directory contains reference notes, separate from the published pages.

```sh
npm run check
npm run build
npm run preview
```

The static site is generated in `dist/`.
