# r-blog

A modular, Rust-powered static blog system featuring Markdown rendering, a local administration backend, and a static frontend optimized for Cloudflare Pages.

## Features

- **Markdown Rendering**: Support for math formulas via KaTeX and syntax highlighting via Highlight.js.
- **Full-text Search**: Built-in site-wide search with a lazy-loaded search index.
- **Table of Contents (TOC)**: Automatically generated outline navigation for both desktop sidebars and mobile drawers.
- **Image & Share Utilities**: Click-to-zoom image lightbox and canvas-based sharing card generator.
- **Incremental Builds**: Fast static page generation with smart incremental writes.
- **Local Admin UI**: Axum-based local web panel to write, edit, and manage Markdown posts.

## Project Structure

- `content/`: Markdown files, assets, and configuration.
- `crates/blog-core`: Core library for parsing Markdown and rendering templates.
- `crates/sitegen`: CLI to build the static site.
- `crates/admin`: Local Axum server for post management.
- `public/`: Generated static assets.

## Commands

### Build the static site

```bash
cargo run -p sitegen -- build
```

### Run the local admin panel

```bash
cargo run -p admin
```

The admin panel will be accessible at `http://127.0.0.1:8787`.

## Deployment

Deploy the `public/` folder to Cloudflare Pages.

1. Write and edit content locally using the admin panel.
2. Push your changes to GitHub.
3. Configure Cloudflare Pages to build using `cargo run -p sitegen -- build` and deploy the output directory (`public/`).
