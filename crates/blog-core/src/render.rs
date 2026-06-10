use crate::content::{Post, SiteConfig};

pub fn stylesheet() -> String {
    String::from(
        r#":root {
  color-scheme: light;
  --bg: #f6f1e7;
  --paper: rgba(255, 253, 248, 0.92);
  --paper-solid: #fffdf8;
  --paper-strong: #fffaf1;
  --text: #20242a;
  --muted: #66707a;
  --accent: #0f766e;
  --accent-strong: #115e59;
  --accent-soft: rgba(15, 118, 110, 0.1);
  --border: #dcd1c2;
  --border-soft: rgba(220, 209, 194, 0.6);
  --code-bg: #f4efe6;
  --shadow: 0 18px 50px rgba(31, 35, 40, 0.06);
  --shadow-strong: 0 22px 70px rgba(31, 35, 40, 0.09);
  --max-width: 980px;
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
    radial-gradient(circle at top left, rgba(15, 118, 110, 0.09), transparent 24%),
    radial-gradient(circle at top right, rgba(17, 94, 89, 0.06), transparent 28%),
    linear-gradient(180deg, #fbf8f2 0%, var(--bg) 100%);
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
    linear-gradient(rgba(255, 255, 255, 0.18) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255, 255, 255, 0.16) 1px, transparent 1px);
  background-size: 72px 72px;
  mask-image: linear-gradient(180deg, rgba(0, 0, 0, 0.16), transparent 35%);
  opacity: 0.22;
}
::selection {
  background: rgba(15, 118, 110, 0.18);
  color: var(--text);
}
a {
  color: var(--accent);
  text-decoration-thickness: 1px;
  text-underline-offset: 0.18em;
}
a:hover { color: var(--accent-strong); }
.wrapper {
  position: relative;
  z-index: 1;
  max-width: var(--max-width);
  margin: 0 auto;
  padding: 28px 20px 72px;
}
.site-header {
  display: flex;
  justify-content: space-between;
  gap: 20px;
  align-items: flex-end;
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
  padding: 22px 0 24px;
  border-bottom: 1px solid var(--border-soft);
  transition: transform 160ms ease, border-color 160ms ease, background-color 160ms ease;
}
.post-card:hover {
  border-bottom-color: rgba(15, 118, 110, 0.22);
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
article {
  max-width: 760px;
  margin: 0 auto;
  padding: 0;
}
article h1 {
  margin: 6px 0 14px;
  font-size: clamp(2rem, 4vw, 3.4rem);
  line-height: 1.12;
  letter-spacing: -0.025em;
}
article h2, article h3 {
  margin: 2em 0 0.7em;
  line-height: 1.2;
}
article h2 {
  font-size: 1.55rem;
}
article h3 {
  font-size: 1.22rem;
}
article p, article ul, article ol, article blockquote, article table {
  margin: 1em 0;
}
article p:first-of-type {
  font-size: 1.05rem;
}
article li + li { margin-top: 0.3em; }
article img {
  max-width: 100%;
  border-radius: 16px;
  display: block;
  margin: 1.4rem 0;
}
article hr {
  border: 0;
  border-top: 1px solid var(--border-soft);
  margin: 2.4rem 0;
}
article blockquote {
  margin-left: 0;
  padding: 0.3rem 0 0.3rem 1.1rem;
  border-left: 3px solid var(--accent);
  color: var(--muted);
  font-style: italic;
}
article pre {
  overflow-x: auto;
  padding: 18px 20px;
  border-radius: 18px;
  background: #191c22;
  color: #f7f7f2;
  border: 1px solid #272c34;
  box-shadow: inset 0 1px 0 rgba(255,255,255,0.04);
}
article pre code {
  background: transparent;
  padding: 0;
  border-radius: 0;
  color: inherit;
}
article code {
  background: var(--code-bg);
  padding: 0.15rem 0.35rem;
  border-radius: 6px;
  font-size: 0.95em;
}
article kbd {
  display: inline-block;
  padding: 0.14rem 0.42rem;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: #fff;
  font-size: 0.88em;
}
article table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.96rem;
  overflow: hidden;
  border-radius: 14px;
}
article th, article td {
  border-bottom: 1px solid var(--border-soft);
  text-align: left;
  padding: 0.65rem 0.35rem;
  vertical-align: top;
}
article thead th {
  background: rgba(15, 118, 110, 0.06);
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

pub fn render_index(config: &SiteConfig, posts: &[Post]) -> String {
    let mut cards = String::new();

    for post in posts.iter().filter(|post| !post.draft()) {
        cards.push_str(&format!(
            r#"<article class="post-card">
  <div class="meta">{date}</div>
  <h2><a href="/posts/{slug}/">{title}</a></h2>
  <p>{description}</p>
</article>"#,
            date = escape_html(post.date()),
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
            title = escape_html(&config.title),
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
            date = escape_html(post.date()),
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
    let math_assets = math_assets();
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="description" content="{description}">
  <title>{title}</title>
  <link rel="stylesheet" href="/styles.css">
  {math_assets}
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
    <footer>Built with Rust and Markdown.</footer>
  </main>
</body>
</html>"#,
        title = escape_html(title),
        description = escape_html(description),
        site_title = escape_html(site_title),
        site_description = escape_html(site_description),
        math_assets = math_assets,
        body = body,
    )
}

fn math_assets() -> String {
    String::from(
        r#"  <link rel="preconnect" href="https://cdn.jsdelivr.net">
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css">
  <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.js"></script>
  <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/contrib/auto-render.min.js" onload="renderMathInElement(document.body, {delimiters: [{left: '$$', right: '$$', display: true}, {left: '$', right: '$', display: false}, {left: '\\[', right: '\\]', display: true}, {left: '\\(', right: '\\)', display: false}], throwOnError: false});"></script>"#,
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
