import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import tailwindcss from '@tailwindcss/vite';
import starlightCatppuccin from '@catppuccin/starlight'
import starlightGitHubAlerts from 'starlight-github-alerts'

export default defineConfig({
  integrations: [
    starlight({
      title: 'Sown',
      description: 'Documentation for Sown, an open platform for interconnected virtual worlds.',
      customCss: ['./src/styles/global.css'],
      sidebar: [
        {label: "Overview", slug: "overview"},
        { label: "Worlds", items: [{ autogenerate: { directory: 'worlds' } }] },
        { label: "Wire", items: [{ autogenerate: { directory: 'wire' } }] },
        { label: "Identity", items: [{ autogenerate: { directory: 'identity' } }] },
      ],
      plugins: [starlightCatppuccin(), starlightGitHubAlerts()],
    }),
  ],
  vite: { plugins: [tailwindcss()] },
});
