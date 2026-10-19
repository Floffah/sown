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

`src/content/docs` is a symlink to the repository's `../records` directory. Edit
documentation there; new Markdown pages need a `title` in YAML frontmatter and
appear automatically in the sidebar. Starlight supplies each page's main heading.

```sh
npm run check
npm run build
npm run preview
```

The static site is generated in `dist/`. Build from the repository checkout so
the linked `../records` directory is available.
