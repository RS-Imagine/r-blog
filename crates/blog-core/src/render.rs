use crate::content::{Post, SiteConfig, Page};

pub fn stylesheet() -> String {
    String::from(
        r#":root {
  color-scheme: light;
  --bg: #DDD6C1;
  --bg-top: #E6DFCB;
  --paper: rgba(221, 214, 193, 0.92);
  --paper-solid: #DDD6C1;
  --paper-strong: #D0C8B3;
  --text: #502010;
  --muted: #807565;
  --accent: #805030;
  --accent-strong: #402008;
  --accent-soft: rgba(128, 80, 48, 0.1);
  --accent-alpha-1: rgba(128, 80, 48, 0.09);
  --accent-alpha-2: rgba(128, 80, 48, 0.06);
  --accent-alpha-3: rgba(128, 80, 48, 0.18);
  --accent-alpha-4: rgba(128, 80, 48, 0.22);
  --border: #C4BCA8;
  --border-soft: rgba(196, 188, 168, 0.6);
  --code-bg: rgba(196, 188, 168, 0.4);
  --shadow: 0 18px 50px rgba(80, 32, 16, 0.08);
  --shadow-strong: 0 22px 70px rgba(80, 32, 16, 0.12);
  --grid-line-1: rgba(255, 255, 255, 0.25);
  --grid-line-2: rgba(255, 255, 255, 0.22);
  --pre-bg: #C4BCA8;
  --pre-text: #502010;
  --pre-border: #C4BCA8;
  --kbd-bg: #E6DFCB;
  --max-width: 980px;
}
@media (prefers-color-scheme: dark) {
  :root:not([data-theme="light"]) {
    color-scheme: dark;
    --bg: #2C201A;
    --bg-top: #33251F;
    --paper: rgba(56, 42, 34, 0.92);
    --paper-solid: #382A22;
    --paper-strong: #423229;
    --text: #F0E6D8;
    --muted: #B8A89A;
    --accent: #E5A080;
    --accent-strong: #FFB591;
    --accent-soft: rgba(229, 160, 128, 0.15);
    --accent-alpha-1: rgba(229, 160, 128, 0.09);
    --accent-alpha-2: rgba(229, 160, 128, 0.06);
    --accent-alpha-3: rgba(229, 160, 128, 0.18);
    --accent-alpha-4: rgba(229, 160, 128, 0.22);
    --border: #5A453A;
    --border-soft: rgba(90, 69, 58, 0.6);
    --code-bg: rgba(90, 69, 58, 0.3);
    --shadow: 0 18px 50px rgba(0, 0, 0, 0.3);
    --shadow-strong: 0 22px 70px rgba(0, 0, 0, 0.4);
    --grid-line-1: rgba(255, 255, 255, 0.03);
    --grid-line-2: rgba(255, 255, 255, 0.02);
    --pre-bg: rgba(25, 18, 14, 0.5);
    --pre-text: #F0E6D8;
    --pre-border: #5A453A;
    --kbd-bg: #423229;
  }
}
:root[data-theme="dark"] {
  color-scheme: dark;
  --bg: #2C201A;
  --bg-top: #33251F;
  --paper: rgba(56, 42, 34, 0.92);
  --paper-solid: #382A22;
  --paper-strong: #423229;
  --text: #F0E6D8;
  --muted: #B8A89A;
  --accent: #E5A080;
  --accent-strong: #FFB591;
  --accent-soft: rgba(229, 160, 128, 0.15);
  --accent-alpha-1: rgba(229, 160, 128, 0.09);
  --accent-alpha-2: rgba(229, 160, 128, 0.06);
  --accent-alpha-3: rgba(229, 160, 128, 0.18);
  --accent-alpha-4: rgba(229, 160, 128, 0.22);
  --border: #5A453A;
  --border-soft: rgba(90, 69, 58, 0.6);
  --code-bg: rgba(90, 69, 58, 0.3);
  --shadow: 0 18px 50px rgba(0, 0, 0, 0.3);
  --shadow-strong: 0 22px 70px rgba(0, 0, 0, 0.4);
  --grid-line-1: rgba(255, 255, 255, 0.03);
  --grid-line-2: rgba(255, 255, 255, 0.02);
  --pre-bg: rgba(25, 18, 14, 0.5);
  --pre-text: #F0E6D8;
  --pre-border: #5A453A;
  --kbd-bg: #423229;
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
  align-items: center;
  gap: 16px;
  padding: 10px 0 14px;
  border-bottom: 1px solid var(--border-soft);
  margin-bottom: 16px;
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
  border: 1px solid var(--border-soft);
}
.article-shell article pre code {
  border: none;
}
.article-shell article pre.hljs {
  background: var(--pre-bg);
}
.article-shell article pre code.hljs {
  background: transparent;
  padding: 0;
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
.zoom-overlay {
  position: fixed;
  top: 0; left: 0; right: 0; bottom: 0;
  background: var(--bg);
  opacity: 0;
  visibility: hidden;
  transition: opacity 0.3s, visibility 0.3s;
  z-index: 100;
}
.zoom-overlay.active {
  opacity: 0.95;
  visibility: visible;
}
article img {
  transition: transform 0.3s ease;
  position: relative;
}
article img.zoomed {
  z-index: 101;
  cursor: zoom-out !important;
}
.toc-container {
  position: fixed;
  left: max(12px, calc(50vw - var(--max-width) / 2 - 230px));
  top: 90px;
  width: 210px;
  max-height: calc(100vh - 110px);
  overflow-y: auto;
  font-size: 0.85rem;
  padding: 16px 12px 16px 0;
  scrollbar-width: thin;
  scrollbar-color: var(--border-soft) transparent;
}
.toc-container::-webkit-scrollbar { width: 3px; }
.toc-container::-webkit-scrollbar-thumb { background: var(--border-soft); border-radius: 3px; }
.toc-title {
  font-weight: 700;
  margin-bottom: 14px;
  color: var(--muted);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-size: 0.75rem;
}
.toc-link {
  display: block;
  color: var(--muted);
  text-decoration: none;
  margin-bottom: 6px;
  line-height: 1.45;
  padding: 2px 0 2px 8px;
  border-left: 2px solid transparent;
  transition: color 0.18s, border-color 0.18s, padding-left 0.18s;
  font-size: 0.83rem;
}
.toc-link:hover {
  color: var(--accent);
  border-left-color: var(--accent-alpha-3);
}
.toc-link.toc-active {
  color: var(--accent);
  font-weight: 600;
  border-left-color: var(--accent);
  padding-left: 10px;
}
.toc-level-2 { margin-left: 0; }
.toc-level-3 { margin-left: 10px; font-size: 0.80rem; }
.toc-level-4 { margin-left: 20px; font-size: 0.78rem; }
.mobile-toc-btn { display: none; }
@media (max-width: 1300px) {
  .toc-container {
    position: fixed;
    top: auto;
    bottom: 0;
    left: 0;
    right: 0;
    width: 100%;
    max-height: 70vh;
    background: var(--paper-solid);
    z-index: 1000;
    padding: 24px 20px calc(24px + env(safe-area-inset-bottom)) 20px;
    border-top-left-radius: 24px;
    border-top-right-radius: 24px;
    box-shadow: var(--shadow-strong);
    transform: translateY(100%);
    transition: transform 0.3s cubic-bezier(0.32, 0.72, 0, 1);
  }
  .toc-container.drawer-open {
    transform: translateY(0);
  }
  .mobile-toc-btn {
    display: flex;
    position: fixed;
    bottom: max(24px, env(safe-area-inset-bottom));
    right: 24px;
    width: 52px;
    height: 52px;
    border-radius: 26px;
    background: var(--accent);
    color: var(--bg);
    align-items: center;
    justify-content: center;
    box-shadow: var(--shadow-strong);
    z-index: 999;
    cursor: pointer;
    border: none;
    transition: transform 0.2s;
  }
  .mobile-toc-btn:active {
    transform: scale(0.95);
  }
}
.search-btn {
  background: none;
  border: none;
  color: var(--accent);
  cursor: pointer;
  font-size: inherit;
  padding: 0;
  font-family: inherit;
  text-decoration: underline;
  text-decoration-thickness: 1px;
  text-underline-offset: 0.18em;
  transition: color 0.2s;
}
.search-btn:hover {
  color: var(--accent-strong);
}
.theme-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  text-decoration: none;
  padding: 0 4px;
}
.sun-icon { display: none; }
.moon-icon { display: block; }
@media (prefers-color-scheme: dark) {
  :root:not([data-theme="light"]) .sun-icon { display: block; }
  :root:not([data-theme="light"]) .moon-icon { display: none; }
}
:root[data-theme="dark"] .sun-icon { display: block; }
:root[data-theme="dark"] .moon-icon { display: none; }
:root[data-theme="light"] .sun-icon { display: none; }
:root[data-theme="light"] .moon-icon { display: block; }
.search-modal {
  position: fixed;
  top: 0; left: 0; right: 0; bottom: 0;
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(4px);
  -webkit-backdrop-filter: blur(4px);
  z-index: 2000;
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding-top: 10vh;
  opacity: 0;
  visibility: hidden;
  transition: opacity 0.2s, visibility 0.2s;
}
.search-modal.active {
  opacity: 1;
  visibility: visible;
}
.search-content {
  background: var(--paper-solid);
  width: 90%;
  max-width: 600px;
  border-radius: 12px;
  box-shadow: var(--shadow-strong);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border: 1px solid var(--border-soft);
}
.search-input-wrapper {
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-soft);
}
.search-input {
  width: 100%;
  border: none;
  background: transparent;
  font-size: 1.1rem;
  color: var(--text);
  outline: none;
  font-family: inherit;
}
.search-input::placeholder {
  color: var(--muted);
}
.search-results {
  max-height: 60vh;
  overflow-y: auto;
  padding: 0;
  margin: 0;
  list-style: none;
}
.search-result-item {
  padding: 12px 20px;
  border-bottom: 1px solid var(--border-soft);
  text-decoration: none;
  display: block;
}
.search-result-item:last-child {
  border-bottom: none;
}
.search-result-item:hover, .search-result-item.selected {
  background: var(--accent-alpha-1);
}
.search-result-title {
  color: var(--accent);
  font-weight: bold;
  font-size: 1rem;
  margin-bottom: 4px;
}
.search-result-excerpt {
  color: var(--muted);
  font-size: 0.85rem;
  line-height: 1.4;
}
.search-result-excerpt mark {
  background: var(--accent-alpha-3);
  color: var(--text);
  padding: 0 2px;
  border-radius: 2px;
}
"#,
    )
}

pub fn favicon_svg() -> String {
    String::from(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 128">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0%" stop-color="#805030"/>
      <stop offset="100%" stop-color="#502010"/>
    </linearGradient>
  </defs>
  <rect width="128" height="128" rx="28" fill="url(#bg)"/>
  <text x="64" y="96" text-anchor="middle" font-family="Georgia, serif" font-size="92" font-weight="700" fill="#E0E0D0">Q</text>
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
        "",
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
  <div class="share-container" style="margin-top: 2rem; padding-top: 1rem; border-top: 1px solid var(--border-soft); display: flex; justify-content: space-between; align-items: center;">
    <a class="back-link" href="/">&larr; Back to posts</a>
    <button id="share-btn" style="background: var(--accent); color: var(--bg); border: none; padding: 8px 16px; border-radius: 6px; cursor: pointer; font-size: 0.95rem; font-weight: 600; display: inline-flex; align-items: center; gap: 6px;">
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
        <path d="M13.5 1a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3zM11 2.5a2.5 2.5 0 1 1 .603 1.628l-6.718 3.12a2.499 2.499 0 0 1 0 1.504l6.718 3.12a2.5 2.5 0 1 1-.488.876l-6.718-3.12a2.5 2.5 0 1 1 0-3.256l6.718-3.12A2.5 2.5 0 0 1 11 2.5zm-8.5 4a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3zm11 5.5a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3z"/>
      </svg>
      Share
    </button>
  </div>
</article></section>"#,
            date = escape_html(&post.updated().map(|u| format!("{} (Updated: {})", post.date(), u)).unwrap_or_else(|| post.date().to_string())),
            title = escape_html(post.title()),
            description = escape_html(post.description()),
            body = post.body_html,
        ),
        &format!(
            r#"<meta property="og:title" content="{title}">
<meta property="og:description" content="{description}">
<meta property="og:type" content="article">"#,
            title = escape_html(post.title()),
            description = escape_html(post.description()),
        ),
    )
}

pub fn render_page(config: &SiteConfig, page_content: &Page) -> String {
    page(
        &page_content.front_matter.title,
        &page_content.front_matter.description,
        &config.title,
        &config.description,
        &format!(
            r#"<section class="article-shell">
  <article>
  <h1>{title}</h1>
  <div>{body}</div>
</article></section>"#,
            title = escape_html(&page_content.front_matter.title),
            body = page_content.body_html,
        ),
        &format!(
            r#"<meta property="og:title" content="{title}">
<meta property="og:description" content="{description}">
<meta property="og:type" content="website">"#,
            title = escape_html(&page_content.front_matter.title),
            description = escape_html(&page_content.front_matter.description),
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
        "",
    )
}

fn page(
  title: &str,
  description: &str,
  site_title: &str,
  site_description: &str,
  body: &str,
  extra_head: &str,
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
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.9.0/build/styles/github.min.css" media="(prefers-color-scheme: light), (prefers-color-scheme: no-preference)">
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.9.0/build/styles/github-dark.min.css" media="(prefers-color-scheme: dark)">
  {extra_head}
  <script>
    (function() {{
      var theme = localStorage.getItem('theme');
      if (theme === 'dark' || theme === 'light') {{
        document.documentElement.setAttribute('data-theme', theme);
      }}
    }})();
  </script>
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
        <button id="site-search-btn" class="search-btn" aria-label="Search">Search</button>
        <a href="/about/">About</a>
        <button id="theme-toggle-btn" class="search-btn theme-btn" aria-label="Toggle Theme">
          <svg class="sun-icon" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"></circle><line x1="12" y1="1" x2="12" y2="3"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line><line x1="1" y1="12" x2="3" y2="12"></line><line x1="21" y1="12" x2="23" y2="12"></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line></svg>
          <svg class="moon-icon" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path></svg>
        </button>
      </nav>
    </header>
    {body}
    <footer>Qiulin built this website using Rust.</footer>
  </main>
  <div id="search-modal" class="search-modal">
    <div class="search-content">
      <div class="search-input-wrapper">
        <input type="text" id="search-input" class="search-input" placeholder="Search posts..." autocomplete="off">
      </div>
      <ul id="search-results" class="search-results"></ul>
    </div>
  </div>
  <script src="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.9.0/build/highlight.min.js"></script>
  <script>
    document.addEventListener('DOMContentLoaded', () => {{
      if (typeof hljs !== 'undefined') {{ hljs.highlightAll(); }}
      
      const themeToggleBtn = document.getElementById('theme-toggle-btn');
      if (themeToggleBtn) {{
        const updateHljsTheme = (theme) => {{
          const hljsLight = document.querySelector('link[href*="github.min.css"]');
          const hljsDark = document.querySelector('link[href*="github-dark.min.css"]');
          if (hljsLight && hljsDark) {{
            if (theme === 'dark') {{
              hljsLight.media = 'none';
              hljsDark.media = 'all';
            }} else {{
              hljsLight.media = 'all';
              hljsDark.media = 'none';
            }}
          }}
        }};

        // Set initial hljs theme if overriden
        const initialTheme = document.documentElement.getAttribute('data-theme');
        if (initialTheme) updateHljsTheme(initialTheme);

        themeToggleBtn.addEventListener('click', () => {{
          const currentTheme = document.documentElement.getAttribute('data-theme') || (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
          const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
          document.documentElement.setAttribute('data-theme', newTheme);
          localStorage.setItem('theme', newTheme);
          updateHljsTheme(newTheme);
        }});
      }}

      const searchBtn = document.getElementById('site-search-btn');
      const searchModal = document.getElementById('search-modal');
      const searchInput = document.getElementById('search-input');
      const searchResults = document.getElementById('search-results');
      let searchIndex = null;

      if (searchBtn && searchModal) {{
        const closeSearch = () => {{
          searchModal.classList.remove('active');
          searchInput.blur();
        }};

        const openSearch = async () => {{
          searchModal.classList.add('active');
          searchInput.focus();
          
          if (!searchIndex) {{
            try {{
              const res = await fetch('/search_index.json');
              searchIndex = await res.json();
            }} catch (e) {{
              console.error('Failed to load search index:', e);
            }}
          }}
        }};

        searchBtn.addEventListener('click', openSearch);
        
        searchModal.addEventListener('click', (e) => {{
          if (e.target === searchModal) closeSearch();
        }});
        
        document.addEventListener('keydown', (e) => {{
          if (e.key === 'Escape' && searchModal.classList.contains('active')) {{
            closeSearch();
          }}
        }});

        searchInput.addEventListener('input', (e) => {{
          const query = e.target.value.trim().toLowerCase();
          searchResults.innerHTML = '';
          
          if (!query || !searchIndex) return;
          
          const results = searchIndex.filter(post => 
            post.title.toLowerCase().includes(query) ||
            post.description.toLowerCase().includes(query) ||
            post.body.toLowerCase().includes(query)
          ).slice(0, 10);
          
          if (results.length === 0) {{
            searchResults.innerHTML = '<li style="padding: 16px 20px; color: var(--muted);">No results found.</li>';
            return;
          }}
          
          results.forEach(post => {{
            const li = document.createElement('li');
            const a = document.createElement('a');
            a.className = 'search-result-item';
            a.href = `/posts/${{post.slug}}/`;
            
            const title = document.createElement('div');
            title.className = 'search-result-title';
            title.textContent = post.title;
            
            let excerptText = post.description;
            const bodyLower = post.body.toLowerCase();
            const matchIndex = bodyLower.indexOf(query);
            
            if (matchIndex !== -1 && !post.description.toLowerCase().includes(query) && !post.title.toLowerCase().includes(query)) {{
              const start = Math.max(0, matchIndex - 40);
              const end = Math.min(post.body.length, matchIndex + query.length + 40);
              excerptText = (start > 0 ? '...' : '') + post.body.substring(start, end) + (end < post.body.length ? '...' : '');
            }}
            
            const excerpt = document.createElement('div');
            excerpt.className = 'search-result-excerpt';
            
            // Highlight query
            const regex = new RegExp(`(${{query.replace(/[.*+?^${{}}()|[\\]\\\\]/g, '\\\\$&')}})`, 'gi');
            excerpt.innerHTML = excerptText.replace(regex, '<mark>$1</mark>');
            title.innerHTML = post.title.replace(regex, '<mark>$1</mark>');
            
            a.appendChild(title);
            a.appendChild(excerpt);
            li.appendChild(a);
            searchResults.appendChild(li);
          }});
        }});
      }}
      const images = document.querySelectorAll('article img');
      const overlay = document.createElement('div');
      overlay.className = 'zoom-overlay';
      document.body.appendChild(overlay);

      images.forEach(img => {{
        img.style.cursor = 'zoom-in';
        img.addEventListener('click', () => {{
          if (img.classList.contains('zoomed')) {{
            img.classList.remove('zoomed');
            overlay.classList.remove('active');
            img.style.transform = '';
          }} else {{
            const rect = img.getBoundingClientRect();
            const x = window.innerWidth / 2 - (rect.left + rect.width / 2);
            const y = window.innerHeight / 2 - (rect.top + rect.height / 2);
            const scale = Math.min(window.innerWidth / rect.width, window.innerHeight / rect.height) * 0.9;
            
            img.classList.add('zoomed');
            overlay.classList.add('active');
            img.style.transform = `translate(${{x}}px, ${{y}}px) scale(${{scale}})`;
          }}
        }});
      }});
      
      overlay.addEventListener('click', () => {{
        const zoomedImg = document.querySelector('img.zoomed');
        if (zoomedImg) {{
          zoomedImg.classList.remove('zoomed');
          zoomedImg.style.transform = '';
        }}
        overlay.classList.remove('active');
      }});
      
      window.addEventListener('scroll', () => {{
        const zoomedImg = document.querySelector('img.zoomed');
        if (zoomedImg) {{
          zoomedImg.classList.remove('zoomed');
          zoomedImg.style.transform = '';
          overlay.classList.remove('active');
        }}
      }});
      
      const shareBtn = document.getElementById('share-btn');
      if (shareBtn) {{
        shareBtn.addEventListener('click', () => {{
          const canvas = document.createElement('canvas');
          const ctx = canvas.getContext('2d');
          const fontStack = "'Times New Roman', 'Noto Serif SC', 'Songti SC', serif";
          const segmenter = new Intl.Segmenter(navigator.language || 'zh-CN', {{ granularity: 'word' }});
          
          function getLines(text, font, maxWidth) {{
             ctx.font = font;
             const words = Array.from(segmenter.segment(text)).map(s => s.segment);
             let lines = [];
             let currentLine = '';
             for (let n = 0; n < words.length; n++) {{
                 const testLine = currentLine + words[n];
                 if (ctx.measureText(testLine).width > maxWidth && n > 0) {{
                     lines.push(currentLine);
                     currentLine = words[n];
                 }} else {{
                     currentLine = testLine;
                 }}
             }}
             if (currentLine) lines.push(currentLine);
             return lines;
          }}

          const title = document.querySelector('article h1').innerText;
          const titleFont = `bold 56px ${{fontStack}}`;
          const titleLines = getLines(title, titleFont, 600);
          
          const desc = document.querySelector('meta[name="description"]')?.content || '';
          const descFont = `bold 32px ${{fontStack}}`;
          const descLines = desc ? getLines(desc, descFont, 600) : [];
          
          const paragraphs = Array.from(document.querySelectorAll('article p:not(.meta)'));
          let bodyText = paragraphs.map(p => p.innerText).join(' ').replace(/\s+/g, ' ').trim();
          if (bodyText.length > 100) bodyText = bodyText.substring(0, 100) + '......';
          const bodyFont = `26px ${{fontStack}}`;
          const bodyLines = bodyText ? getLines(bodyText, bodyFont, 600) : [];
          
          let contentHeight = 140; 
          contentHeight += titleLines.length * 76;
          contentHeight += 20; 
          
          if (descLines.length > 0) {{
              contentHeight += descLines.length * 48;
              contentHeight += 40;
          }}
          
          if (bodyLines.length > 0) {{
              contentHeight += bodyLines.length * 42;
              contentHeight += 60;
          }}
          
          const footerHeight = 160;
          const height = contentHeight + footerHeight;
          const width = 800;
          
          canvas.width = width * 2;
          canvas.height = height * 2;
          canvas.style.width = width + 'px';
          canvas.style.height = height + 'px';
          ctx.scale(2, 2);

          const isDark = document.documentElement.getAttribute('data-theme') === 'dark' || (document.documentElement.getAttribute('data-theme') !== 'light' && window.matchMedia('(prefers-color-scheme: dark)').matches);
          ctx.fillStyle = isDark ? '#2C201A' : '#DDD6C1';
          ctx.fillRect(0, 0, width, height);

          ctx.fillStyle = isDark ? 'rgba(229, 160, 128, 0.09)' : 'rgba(128, 80, 48, 0.09)';
          ctx.beginPath();
          ctx.arc(0, 0, 300, 0, Math.PI * 2);
          ctx.fill();
          
          ctx.fillStyle = isDark ? 'rgba(229, 160, 128, 0.06)' : 'rgba(128, 80, 48, 0.06)';
          ctx.beginPath();
          ctx.arc(width, 0, 400, 0, Math.PI * 2);
          ctx.fill();
          
          let drawY = 140;
          
          ctx.fillStyle = isDark ? '#F0E6D8' : '#502010';
          ctx.font = titleFont;
          for (let line of titleLines) {{
              ctx.fillText(line, 100, drawY);
              drawY += 76;
          }}
          drawY += 20;
          
          if (descLines.length > 0) {{
              ctx.fillStyle = isDark ? '#E5A080' : '#805030';
              ctx.font = descFont;
              for (let line of descLines) {{
                  ctx.fillText(line, 100, drawY);
                  drawY += 48;
              }}
              drawY += 40;
          }}
          
          if (bodyLines.length > 0) {{
              ctx.fillStyle = isDark ? '#B8A89A' : '#807565';
              ctx.font = bodyFont;
              for (let line of bodyLines) {{
                  ctx.fillText(line, 100, drawY);
                  drawY += 42;
              }}
              drawY += 60;
          }}
          
          const siteName = document.querySelector('.site-brand h1').innerText;
          const domain = window.location.host + window.location.pathname;
          
          ctx.fillStyle = isDark ? '#5A453A' : '#C4BCA8';
          ctx.fillRect(100, height - 150, 600, 2);

          ctx.font = `bold 28px ${{fontStack}}`;
          ctx.fillStyle = isDark ? '#F0E6D8' : '#502010';
          ctx.fillText(siteName, 100, height - 100);
          
          ctx.font = `italic 24px ${{fontStack}}`;
          ctx.fillStyle = isDark ? '#B8A89A' : '#807565';
          ctx.fillText(domain, 100, height - 60);

          const dataUrl = canvas.toDataURL('image/png');
          
          const cardOverlay = document.createElement('div');
          cardOverlay.className = 'zoom-overlay active';
          cardOverlay.style.display = 'flex';
          cardOverlay.style.flexDirection = 'column';
          cardOverlay.style.alignItems = 'center';
          cardOverlay.style.justifyContent = 'center';
          
          const imgElement = document.createElement('img');
          imgElement.src = dataUrl;
          imgElement.style.maxWidth = '85%';
          imgElement.style.maxHeight = '75vh';
          imgElement.style.borderRadius = '16px';
          imgElement.style.boxShadow = 'var(--shadow-strong)';
          
          const tip = document.createElement('p');
          tip.innerText = '长按或右键保存图片 (Long press or right click to save)';
          tip.style.color = 'var(--text)';
          tip.style.marginTop = '20px';
          tip.style.background = 'var(--paper)';
          tip.style.padding = '8px 16px';
          tip.style.borderRadius = '8px';
          tip.style.fontSize = '0.9rem';
          
          cardOverlay.appendChild(imgElement);
          cardOverlay.appendChild(tip);
          document.body.appendChild(cardOverlay);
          
          cardOverlay.addEventListener('click', (e) => {{
              if(e.target === cardOverlay) {{
                  cardOverlay.remove();
              }}
          }});
        }});
      }}
      
      const article = document.querySelector('.article-shell article');
      if (article) {{
        const headings = article.querySelectorAll('h2, h3, h4');
        if (headings.length > 0) {{
          const tocContainer = document.createElement('div');
          tocContainer.className = 'toc-container';
          
          const tocTitle = document.createElement('div');
          tocTitle.className = 'toc-title';
          tocTitle.innerText = 'Contents';
          tocContainer.appendChild(tocTitle);
          
          headings.forEach((heading, index) => {{
            let id = heading.id;
            if (!id) {{
                id = heading.innerText.replace(/[\s]+/g, '-').replace(/[^\w\u4e00-\u9fa5\-]+/g, '').toLowerCase();
                if (!id) id = 'heading-' + index;
                heading.id = id;
            }}
            
            const link = document.createElement('a');
            link.href = '#' + id;
            link.className = 'toc-link toc-level-' + heading.tagName.charAt(1);
            link.innerText = heading.innerText;
            tocContainer.appendChild(link);
          }});
          
          document.body.appendChild(tocContainer);

          // Mobile TOC Button & Drawer Logic
          const mobileTocBtn = document.createElement('button');
          mobileTocBtn.className = 'mobile-toc-btn';
          mobileTocBtn.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 16 16"><path fill-rule="evenodd" d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5z"/></svg>';
          document.body.appendChild(mobileTocBtn);

          const tocDrawerOverlay = document.createElement('div');
          tocDrawerOverlay.className = 'zoom-overlay';
          tocDrawerOverlay.style.zIndex = '998';
          document.body.appendChild(tocDrawerOverlay);

          function closeTocDrawer() {{
             tocContainer.classList.remove('drawer-open');
             tocDrawerOverlay.classList.remove('active');
          }}

          mobileTocBtn.addEventListener('click', () => {{
            if (tocContainer.classList.contains('drawer-open')) {{
              closeTocDrawer();
            }} else {{
              tocContainer.classList.add('drawer-open');
              tocDrawerOverlay.classList.add('active');
            }}
          }});

          tocDrawerOverlay.addEventListener('click', closeTocDrawer);

          // Highlight active TOC entry on scroll
          const tocLinks = tocContainer.querySelectorAll('.toc-link');
          tocLinks.forEach(l => l.addEventListener('click', closeTocDrawer));
          const headingEls = Array.from(headings);

          function setActive(id) {{
            tocLinks.forEach(l => {{
              if (l.getAttribute('href') === '#' + id) {{
                l.classList.add('toc-active');
                // Scroll the TOC so the active item stays visible
                const containerRect = tocContainer.getBoundingClientRect();
                const linkRect = l.getBoundingClientRect();
                if (linkRect.top < containerRect.top + 20) {{
                  tocContainer.scrollTop -= (containerRect.top + 20 - linkRect.top);
                }} else if (linkRect.bottom > containerRect.bottom - 20) {{
                  tocContainer.scrollTop += (linkRect.bottom - containerRect.bottom + 20);
                }}
              }} else {{
                l.classList.remove('toc-active');
              }}
            }});
          }}

          const observer = new IntersectionObserver(
            (entries) => {{
              // Find the topmost heading that is intersecting or has passed
              let best = null;
              let bestTop = Infinity;
              entries.forEach(entry => {{
                if (entry.isIntersecting) {{
                  const top = entry.boundingClientRect.top;
                  if (top < bestTop) {{
                    bestTop = top;
                    best = entry.target;
                  }}
                }}
              }});
              if (best) {{
                setActive(best.id);
              }}
            }},
            {{
              rootMargin: '0px 0px -60% 0px',
              threshold: 0,
            }}
          );

          headingEls.forEach(h => observer.observe(h));

          // Fallback: on scroll, find the heading closest to viewport top
          let ticking = false;
          window.addEventListener('scroll', () => {{
            if (!ticking) {{
              requestAnimationFrame(() => {{
                const scrollY = window.scrollY + 100;
                let active = headingEls[0];
                for (const h of headingEls) {{
                  if (h.offsetTop <= scrollY) active = h;
                  else break;
                }}
                if (active) setActive(active.id);
                ticking = false;
              }});
              ticking = true;
            }}
          }});

          // Set initial active on load
          if (headingEls[0]) setActive(headingEls[0].id);
        }}
      }}
    }});
  </script>
</body>
</html>"#,
        title = escape_html(title),
        description = escape_html(description),
        site_title = escape_html(site_title),
        site_description = escape_html(site_description),
        body = body,
        extra_head = extra_head,
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
