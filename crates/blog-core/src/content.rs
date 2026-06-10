use anyhow::{bail, Context, Result};
use katex::{render_to_string, KatexContext, Settings};
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SiteConfig {
    pub title: String,
    #[serde(rename = "bigTitle", default)]
    pub big_title: Option<String>,
    pub subtitle: String,
    pub author: String,
    pub description: String,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: "r-blog".to_string(),
            big_title: None,
            subtitle: "Rust, Markdown, and Cloudflare Pages".to_string(),
            author: "Your Name".to_string(),
            description: "A personal blog built from Markdown content.".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: String,
    #[serde(default)]
    pub updated: Option<String>,
    pub slug: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub draft: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostDraft {
    pub title: String,
    pub date: String,
    pub updated: Option<String>,
    pub slug: String,
    pub description: String,
    #[serde(default)]
    pub draft: bool,
    pub body_markdown: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Post {
    pub front_matter: FrontMatter,
    pub body_markdown: String,
    pub body_html: String,
}

impl Post {
    pub fn title(&self) -> &str {
        &self.front_matter.title
    }

    pub fn slug(&self) -> &str {
        &self.front_matter.slug
    }

    pub fn date(&self) -> &str {
        &self.front_matter.date
    }

    pub fn updated(&self) -> Option<&str> {
        self.front_matter.updated.as_deref()
    }

    pub fn description(&self) -> &str {
        &self.front_matter.description
    }

    pub fn draft(&self) -> bool {
        self.front_matter.draft
    }
}

pub fn load_site_config(content_root: impl AsRef<Path>) -> Result<SiteConfig> {
    let path = content_root.as_ref().join("site.toml");
    if !path.exists() {
        return Ok(SiteConfig::default());
    }

    let raw = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let config = toml::from_str(&raw).with_context(|| format!("parse {}", path.display()))?;
    Ok(config)
}

pub fn load_posts(content_root: impl AsRef<Path>) -> Result<Vec<Post>> {
    let posts_dir = content_root.as_ref().join("posts");
    let mut posts = Vec::new();

    if !posts_dir.exists() {
        return Ok(posts);
    }

    for entry in fs::read_dir(&posts_dir).with_context(|| format!("read {}", posts_dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }
        posts.push(load_post_file(&path)?);
    }

    posts.sort_by(|left, right| {
        let l_time = left.updated().unwrap_or(left.date());
        let r_time = right.updated().unwrap_or(right.date());
        r_time.cmp(l_time).then_with(|| right.slug().cmp(left.slug()))
    });
    Ok(posts)
}

pub fn load_post_file(path: impl AsRef<Path>) -> Result<Post> {
    let path = path.as_ref();
    let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let (front_matter, body_markdown) = split_front_matter(&raw)?;
    let front_matter: FrontMatter = toml::from_str(&front_matter)
        .with_context(|| format!("parse front matter in {}", path.display()))?;
    let body_html = markdown_to_html(&body_markdown);

    Ok(Post {
        front_matter,
        body_markdown,
        body_html,
    })
}

pub fn load_post_by_slug(content_root: impl AsRef<Path>, slug: &str) -> Result<Option<Post>> {
    let path = post_path(content_root, slug);
    if !path.exists() {
        return Ok(None);
    }

    load_post_file(path).map(Some)
}

pub fn save_post(content_root: impl AsRef<Path>, draft: &PostDraft) -> Result<PathBuf> {
    let content_root = content_root.as_ref();
    let posts_dir = content_root.join("posts");
    fs::create_dir_all(&posts_dir)?;

    let front_matter = FrontMatter {
        title: draft.title.clone(),
        date: draft.date.clone(),
        updated: draft.updated.clone(),
        slug: draft.slug.clone(),
        description: draft.description.clone(),
        draft: draft.draft,
    };

    let file_path = posts_dir.join(format!("{}.md", front_matter.slug));
    let serialized = toml::to_string_pretty(&front_matter)?;
    let content = format!("+++
{}
+++

{}
", serialized.trim(), draft.body_markdown.trim_start());
    fs::write(&file_path, content).with_context(|| format!("write {}", file_path.display()))?;
    Ok(file_path)
}

pub fn delete_post(content_root: impl AsRef<Path>, slug: &str) -> Result<bool> {
    let path = post_path(content_root, slug);
    if !path.exists() {
        return Ok(false);
    }

    fs::remove_file(&path).with_context(|| format!("delete {}", path.display()))?;
    Ok(true)
}

pub fn post_path(content_root: impl AsRef<Path>, slug: &str) -> PathBuf {
    content_root.as_ref().join("posts").join(format!("{}.md", slug))
}

pub fn slugify(input: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    let trimmed = slug.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "post".to_string()
    } else {
        trimmed
    }
}

fn split_front_matter(source: &str) -> Result<(String, String)> {
    let mut lines = source.lines();
    let first = lines.next().context("missing front matter start")?;
    if first.trim() != "+++" {
        bail!("front matter must start with +++");
    }

    let mut front_matter = String::new();
    let mut body = String::new();
    let mut in_front_matter = true;

    for line in lines {
        if in_front_matter && line.trim() == "+++" {
            in_front_matter = false;
            continue;
        }

        if in_front_matter {
            front_matter.push_str(line);
            front_matter.push('\n');
        } else {
            body.push_str(line);
            body.push('\n');
        }
    }

    if in_front_matter {
        bail!("front matter must end with +++");
    }

    Ok((front_matter, body.trim_start_matches('\n').to_string()))
}

fn markdown_to_html(markdown: &str) -> String {
    let (processed_markdown, math_segments) = preprocess_math(markdown);
    let parser = Parser::new_ext(&processed_markdown, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    for segment in math_segments {
        let rendered = render_math_expr(&segment.math, segment.display_mode);
        html_output = html_output.replace(&segment.placeholder, &rendered);
        if segment.display_mode {
            html_output = html_output.replace(
                &format!("<p>{}</p>", segment.placeholder),
                &format!("<div class=\"math-block\">{rendered}</div>"),
            );
        }
    }

    html_output
}

#[derive(Debug, Clone)]
struct MathSegment {
    placeholder: String,
    math: String,
    display_mode: bool,
}

fn preprocess_math(markdown: &str) -> (String, Vec<MathSegment>) {
    let mut output = String::new();
    let mut in_fenced_code_block = false;
    let mut in_block_math = false;
    let mut block_math = String::new();
    let mut math_segments = Vec::new();
    let mut math_counter = 0usize;

    for line in markdown.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            if !in_block_math {
                in_fenced_code_block = !in_fenced_code_block;
            }
            output.push_str(line);
            output.push('\n');
            continue;
        }

        if in_fenced_code_block {
            output.push_str(line);
            output.push('\n');
            continue;
        }

        if trimmed == "$$" {
            if in_block_math {
                let placeholder = format!("<div class=\"math-block\" data-math-placeholder=\"{}\"></div>", math_counter);
                math_segments.push(MathSegment {
                    placeholder: placeholder.clone(),
                    math: block_math.trim().to_string(),
                    display_mode: true,
                });
                math_counter += 1;
                output.push_str(&placeholder);
                output.push('\n');
                block_math.clear();
                in_block_math = false;
            } else {
                in_block_math = true;
            }
            continue;
        }

        if in_block_math {
            block_math.push_str(line);
            block_math.push('\n');
            continue;
        }

        if trimmed.starts_with("$$") && trimmed.ends_with("$$") && trimmed.len() > 4 {
            let inner = trimmed.trim_start_matches("$$").trim_end_matches("$$").trim();
            let placeholder = format!("<div class=\"math-block\" data-math-placeholder=\"{}\"></div>", math_counter);
            math_segments.push(MathSegment {
                placeholder: placeholder.clone(),
                math: inner.to_string(),
                display_mode: true,
            });
            math_counter += 1;
            output.push_str(&placeholder);
            output.push('\n');
            continue;
        }

        output.push_str(&render_inline_math(line, &mut math_segments, &mut math_counter));
        output.push('\n');
    }

    if in_block_math {
        output.push_str("$$\n");
        output.push_str(&block_math);
    }

    (output, math_segments)
}

fn render_inline_math(
    line: &str,
    math_segments: &mut Vec<MathSegment>,
    math_counter: &mut usize,
) -> String {
    let chars: Vec<(usize, char)> = line.char_indices().collect();
    let mut output = String::new();
    let mut index = 0;
    let mut in_inline_code = false;

    while index < chars.len() {
        let (_, ch) = chars[index];

        if ch == '`' {
            in_inline_code = !in_inline_code;
            output.push(ch);
            index += 1;
            continue;
        }

        if ch == '\\' {
            output.push(ch);
            if let Some((_, next)) = chars.get(index + 1) {
                output.push(*next);
                index += 2;
            } else {
                index += 1;
            }
            continue;
        }

        if ch == '$' && !in_inline_code {
            if chars.get(index + 1).map(|(_, next)| *next) == Some('$') {
                output.push_str("$$");
                index += 2;
                continue;
            }

            let mut math = String::new();
            let mut lookahead = index + 1;
            let mut found_closing = false;

            while lookahead < chars.len() {
                let (_, next_ch) = chars[lookahead];

                if next_ch == '\\' {
                    math.push(next_ch);
                    if let Some((_, escaped)) = chars.get(lookahead + 1) {
                        math.push(*escaped);
                        lookahead += 2;
                    } else {
                        lookahead += 1;
                    }
                    continue;
                }

                if next_ch == '$' {
                    found_closing = true;
                    break;
                }

                math.push(next_ch);
                lookahead += 1;
            }

            if found_closing {
                let placeholder = format!("<span class=\"math math-inline\" data-math-placeholder=\"{}\"></span>", *math_counter);
                math_segments.push(MathSegment {
                    placeholder: placeholder.clone(),
                    math,
                    display_mode: false,
                });
                *math_counter += 1;
                output.push_str(&placeholder);
                index = lookahead + 1;
                continue;
            }
        }

        output.push(ch);
        index += 1;
    }

    output
}

fn render_math_expr(math: &str, display_mode: bool) -> String {
    let ctx = KatexContext::default();
    let mut settings = Settings::default();
    settings.display_mode = display_mode;
    settings.throw_on_error = false;

    render_to_string(&ctx, math, &settings).unwrap_or_else(|_| format!("<code>{}</code>", escape_html(math)))
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
