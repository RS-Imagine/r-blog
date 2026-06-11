use crate::content::{Post, SiteConfig};

pub fn stylesheet() -> String {
    String::from(
        r#":root {
  color-scheme: light;
  --bg: #f6f1e7;
  --bg-top: #fbf8f2;
  --paper: rgba(255, 253, 248, 0.92);
  --paper-solid: #fffdf8;
  --paper-strong: #fffaf1;
  --text: #20242a;
  --muted: #66707a;
  --accent: #794823;
  --accent-strong: #5c3518;
  --accent-soft: rgba(121, 72, 35, 0.1);
  --accent-alpha-1: rgba(121, 72, 35, 0.09);
  --accent-alpha-2: rgba(121, 72, 35, 0.06);
  --accent-alpha-3: rgba(121, 72, 35, 0.18);
  --accent-alpha-4: rgba(121, 72, 35, 0.22);
  --border: #dcd1c2;
  --border-soft: rgba(220, 209, 194, 0.6);
  --code-bg: #f4efe6;
  --shadow: 0 18px 50px rgba(31, 35, 40, 0.06);
  --shadow-strong: 0 22px 70px rgba(31, 35, 40, 0.09);
  --grid-line-1: rgba(255, 255, 255, 0.18);
  --grid-line-2: rgba(255, 255, 255, 0.16);
  --pre-bg: #1a1816;
  --pre-text: #f7f5f2;
  --pre-border: #2a2622;
  --kbd-bg: #fff;
  --max-width: 980px;
}
@media (prefers-color-scheme: dark) {
  :root {
    color-scheme: dark;
    --bg: #1c1a17;
    --bg-top: #24221e;
    --paper: rgba(30, 28, 25, 0.92);
    --paper-solid: #1e1c19;
    --paper-strong: #262420;
    --text: #e6e0d8;
    --muted: #a39c93;
    --accent: #c48a5c;
    --accent-strong: #d9a073;
    --accent-soft: rgba(196, 138, 92, 0.15);
    --accent-alpha-1: rgba(196, 138, 92, 0.09);
    --accent-alpha-2: rgba(196, 138, 92, 0.06);
    --accent-alpha-3: rgba(196, 138, 92, 0.18);
    --accent-alpha-4: rgba(196, 138, 92, 0.22);
    --border: #3d3832;
    --border-soft: rgba(61, 56, 50, 0.6);
    --code-bg: #262420;
    --shadow: 0 18px 50px rgba(0, 0, 0, 0.3);
    --shadow-strong: 0 22px 70px rgba(0, 0, 0, 0.4);
    --grid-line-1: rgba(255, 255, 255, 0.04);
    --grid-line-2: rgba(255, 255, 255, 0.03);
    --pre-bg: #151311;
    --pre-text: #e6e0d8;
    --pre-border: #262420;
    --kbd-bg: #262420;
  }
}
* { box-sizing: border-box; }
html {
  -webkit-text-size-adjust: 100%;
  text-rendering: optimizeLegibility;
  scroll-behavior: smooth;
}
html, body { margin: 0; padding: 0; }
body {
  font-family:
    'Noto Serif SC',
    'Source Han Serif SC',
    'Songti SC',
    'STSong',
    'Iowan Old Style',
    'Palatino Linotype',
    'Times New Roman',
    serif;
  background:
    radial-gradient(circle at top left, var(--accent-alpha-1), transparent 24%),
    radial-gradient(circle at top right, var(--accent-alpha-2), transparent 28%),
    linear-gradient(180deg, var(--bg-top) 0%, var(--bg) 100%);
  color: var(--text);
  line-height: 1.85;
  font-size: 18px;
  font-variant-ligatures: common-ligatures;
  font-feature-settings: 'kern' 1, 'liga' 1, 'calt' 1;
}
body::before {
  content: '';
  position: fixed;
  inset: 0;
  pointer-events: none;
  background-image:
    linear-gradient(var(--grid-line-1) 1px, transparent 1px),
    linear-gradient(90deg, var(--grid-line-2) 1px, transparent 1px);
  background-size: 72px 72px;
  mask-image: linear-gradient(180deg, rgba(0, 0, 0, 0.16), transparent 35%);
  opacity: 0.22;
}
::selection {
  background: var(--accent-alpha-3);
  color: var(--text);
}
a {
  color: var(--accent);
  text-decoration-thickness: 1px;
  text-underline-offset: 0.18em;
}
a:hover { color: var(--accent-strong); }
.wrapper {
  z-index: 1;
  max-width: var(--max-width);
  margin: 0 auto;
  padding: 28px 20px 72px;
}
.site-header {
  display: flex;
  justify-content: space-between;
  gap: 20px;
  padding: 18px 0 24px;
  border-bottom: 1px solid var(--border-soft);
  margin-bottom: 28px;
}
.site-brand {
  display: grid;
  gap: 8px;
}
.site-brand h1 {
  margin: 0;
  font-size: 1.05rem;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}
.site-brand p {
  margin: 0;
  max-width: 54ch;
  color: var(--muted);
  font-size: 0.95rem;
}
.site-nav {
  display: flex;
  gap: 14px;
  flex-wrap: wrap;
  justify-content: flex-end;
  color: var(--muted);
  font-size: 0.92rem;
}
.hero {
  padding: 28px 0 30px;
  margin-bottom: 18px;
  border-bottom: 1px solid var(--border-soft);
}
.hero h1 {
  margin: 8px 0 14px;
  font-size: clamp(2.3rem, 6vw, 4.7rem);
  line-height: 1.04;
  letter-spacing: -0.03em;
}
.hero p {
  margin: 0;
  max-width: 66ch;
  color: var(--muted);
  font-size: 1.02rem;
}
.grid { display: grid; gap: 0; }
.post-card {
  width: 100%;
  padding: 22px 0 24px;
  border-bottom: 1px solid var(--border-soft);
  transition: transform 160ms ease, border-color 160ms ease, background-color 160ms ease;
}
.post-card:hover {
  border-bottom-color: var(--accent-alpha-4);
  transform: translateY(-1px);
}
.post-card:last-child { border-bottom: 0; }
.post-card h2 {
  margin: 4px 0 8px;
  font-size: 1.45rem;
  line-height: 1.22;
  font-weight: 700;
  letter-spacing: -0.01em;
}
.post-card p { margin: 0; color: var(--muted); max-width: 70ch; }
.meta {
  color: var(--muted);
  font-size: 0.88rem;
  letter-spacing: 0.02em;
}
.tag {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  color: var(--accent);
  font-size: 0.82rem;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.08em;
}
.tag::before {
  content: '';
  width: 26px;
  height: 1px;
  background: var(--accent);
}
.article-shell {
  padding: 10px 0 0;
}
.article-shell article {
  max-width: 760px;
  margin: 0 auto;
  padding: 0;
}
.article-shell article h1 {
  margin: 6px 0 14px;
  font-size: clamp(2rem, 4vw, 3.4rem);
  line-height: 1.12;
  letter-spacing: -0.025em;
}
.article-shell article h2, .article-shell article h3 {
  margin: 2em 0 0.7em;
  line-height: 1.2;
}
.article-shell article h2 {
  font-size: 1.55rem;
}
.article-shell article h3 {
  font-size: 1.22rem;
}
.article-shell article p, .article-shell article ul, .article-shell article ol, .article-shell article blockquote, .article-shell article table {
  margin: 1em 0;
}
.article-shell article p:first-of-type {
  font-size: 1.05rem;
}
.article-shell article li + li { margin-top: 0.3em; }
.article-shell article img {
  max-width: 100%;
  border-radius: 16px;
  display: block;
  margin: 1.4rem 0;
}
.article-shell article hr {
  border: 0;
  border-top: 1px solid var(--border-soft);
  margin: 2.4rem 0;
}
.article-shell article blockquote {
  margin-left: 0;
  padding: 0.3rem 0 0.3rem 1.1rem;
  border-left: 3px solid var(--accent);
  color: var(--muted);
  font-style: italic;
}
.article-shell article pre {
  overflow-x: auto;
  padding: 18px 20px;
  border-radius: 18px;
  background: var(--pre-bg);
  color: var(--pre-text);
  border: 1px solid var(--pre-border);
  box-shadow: inset 0 1px 0 rgba(255,255,255,0.04);
}
.article-shell article pre code {
  background: transparent;
  padding: 0;
  border-radius: 0;
  color: inherit;
}
.article-shell article code {
  background: var(--code-bg);
  padding: 0.15rem 0.35rem;
  border-radius: 6px;
  font-size: 0.95em;
}
.article-shell article .katex,
.article-shell article .katex-display {
  font-size: 1em;
}
.article-shell article .katex-display {
  margin: 1.1rem 0;
  overflow-x: auto;
  overflow-y: hidden;
}
.article-shell article .katex-display > .katex {
  font-size: 1.02em;
}
.article-shell article kbd {
  display: inline-block;
  padding: 0.14rem 0.42rem;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--kbd-bg);
  font-size: 0.88em;
}
.article-shell article table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.96rem;
  overflow: hidden;
  border-radius: 14px;
}
.article-shell article th, .article-shell article td {
  border-bottom: 1px solid var(--border-soft);
  text-align: left;
  padding: 0.65rem 0.35rem;
  vertical-align: top;
}
.article-shell article thead th {
  background: var(--accent-alpha-2);
  color: var(--accent-strong);
}
footer {
  margin-top: 44px;
  padding-top: 18px;
  border-top: 1px solid var(--border-soft);
  color: var(--muted);
  font-size: 0.9rem;
}
.math {
  font-family: 'KaTeX_Main', 'STIX Two Math', 'Cambria Math', serif;
}
.back-link {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  color: var(--muted);
  font-size: 0.94rem;
}
.back-link:hover { color: var(--accent); }
@media (max-width: 720px) {
  body { font-size: 17px; }
  .wrapper { padding: 20px 14px 52px; }
  .site-header { flex-direction: column; align-items: flex-start; }
  .site-nav { justify-content: flex-start; }
  .hero h1, article h1 { letter-spacing: -0.01em; }
}
"#,
    )
}

pub fn favicon_svg() -> String {
    String::from(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 128">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0%" stop-color="#aa6321ff"/>
      <stop offset="100%" stop-color="#a15600ff"/>
    </linearGradient>
  </defs>
  <rect width="128" height="128" rx="28" fill="url(#bg)"/>
  <text x="64" y="96" text-anchor="middle" font-family="Georgia, serif" font-size="92" font-weight="700" fill="#fffdf8">Q</text>
</svg>"##,
    )
}

pub fn render_index(config: &SiteConfig, posts: &[Post]) -> String {
    let mut cards = String::new();

    for post in posts.iter().filter(|post| !post.draft()) {
        let date_str = if let Some(updated) = post.updated() {
            format!("{} (Updated: {})", post.date(), updated)
        } else {
            post.date().to_string()
        };

        cards.push_str(&format!(
            r#"<article class="post-card">
  <div class="meta">{date}</div>
  <h2><a href="/posts/{slug}/">{title}</a></h2>
  <p>{description}</p>
</article>"#,
            date = escape_html(&date_str),
            slug = escape_html(post.slug()),
            title = escape_html(post.title()),
            description = escape_html(post.description()),
        ));
    }

    page(
      &config.title,
      &config.description,
      &config.title,
      &config.description,
        &format!(
            r#"<section class="hero">
  <span class="tag">{author}</span>
  <h1>{title}</h1>
  <p>{subtitle}</p>
</section>
<section class="grid">
  {cards}
</section>"#,
            author = escape_html(&config.author),
            title = escape_html(config.big_title.as_deref().unwrap_or(&config.title)),
            subtitle = escape_html(&config.subtitle),
            cards = cards,
        ),
    )
}

pub fn render_post(config: &SiteConfig, post: &Post) -> String {
    page(
        &post.front_matter.title,
        &post.front_matter.description,
    &config.title,
    &config.description,
        &format!(
            r#"<section class="article-shell">
  <a class="back-link" href="/">&larr; Back to posts</a>
  <article>
  <div class="meta">{date}</div>
  <h1>{title}</h1>
  <p class="meta">{description}</p>
  <div>{body}</div>
</article>"#,
            date = escape_html(&post.updated().map(|u| format!("{} (Updated: {})", post.date(), u)).unwrap_or_else(|| post.date().to_string())),
            title = escape_html(post.title()),
            description = escape_html(post.description()),
            body = post.body_html,
        ),
    )
}

pub fn render_404(config: &SiteConfig) -> String {
    page(
        "Not Found",
        &config.description,
        &config.title,
        &config.description,
        r#"<article>
  <h1>404</h1>
  <p>The page you are looking for does not exist.</p>
  <a href="/">Go back home</a>
</article>"#,
    )
}

fn page(
  title: &str,
  description: &str,
  site_title: &str,
  site_description: &str,
  body: &str,
) -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="description" content="{description}">
  <title>{title}</title>
  <link rel="stylesheet" href="/styles.css">
  <link rel="icon" type="image/svg+xml" href="/favicon.svg">
  <link rel="preconnect" href="https://cdn.jsdelivr.net">
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css">
</head>
<body>
  <main class="wrapper">
    <header class="site-header">
      <div class="site-brand">
        <h1>{site_title}</h1>
        <p>{site_description}</p>
      </div>
      <nav class="site-nav" aria-label="Primary">
        <a href="/">Home</a>
      </nav>
    </header>
    {body}
    <footer>Qiulin built this website using Rust.</footer>
  </main>
</body>
</html>"#,
        title = escape_html(title),
        description = escape_html(description),
        site_title = escape_html(site_title),
        site_description = escape_html(site_description),
        body = body,
    )
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
