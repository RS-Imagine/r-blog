# r-blog

A Rust-powered personal blog with Markdown content, a local-only admin backend, and a static frontend for Cloudflare Pages.

The generated site now uses a typography-first layout with serif Chinese/English font stacks and KaTeX-powered math rendering for Markdown formulas.

## Layout

- `content/`: Markdown posts and site config
- `crates/blog-core`: shared content loading and static site generation
- `crates/sitegen`: CLI that builds the public site
- `crates/admin`: local-only Axum admin app with post create/edit/delete/rebuild flows
- `public/`: generated static site output

## Commands

Build the site:

```bash
cargo run -p sitegen -- build
```

Run the local admin app:

```bash
cargo run -p admin
```

The admin app runs on `127.0.0.1:8787` and provides a browser UI for creating, editing, deleting, and rebuilding Markdown posts.

## Cloudflare Pages

The generated `public/` directory is the deploy artifact for Cloudflare Pages.

## Publishing Flow

Recommended flow:

1. Edit Markdown content locally under `content/posts/`.
2. Rebuild the static site with `cargo run -p sitegen -- build` if you want to verify the output locally.
3. Commit and push the changes to GitHub.
4. Let Cloudflare Pages build from the GitHub repository automatically.

With this setup, the local admin app only changes files in the repo. GitHub receives the commit, and Cloudflare rebuilds from that commit without needing the admin app to be public.

If you want the backend to push commits for you later, that can be added as a separate step, but it is not required for the free-tier workflow.
