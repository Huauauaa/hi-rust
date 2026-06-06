# hi-rust

This repository contains a VitePress documentation site powered by pnpm.

## Development

Install dependencies:

```sh
pnpm install
```

Start the documentation server:

```sh
pnpm docs:dev
```

Build the static site:

```sh
pnpm docs:build
```

Preview the built site:

```sh
pnpm docs:preview
```

## GitHub Pages

The GitHub Actions workflow in `.github/workflows/deploy-pages.yml`
builds the VitePress site and deploys `docs/.vitepress/dist` to
GitHub Pages whenever changes are pushed to `main`.
