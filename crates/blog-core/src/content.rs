use anyhow::{bail, Context, Result};
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SiteConfig {
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub description: String,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: "r-blog".to_string(),
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

    posts.sort_by(|left, right| right.date().cmp(left.date()).then_with(|| right.slug().cmp(left.slug())));
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
    let parser = Parser::new_ext(markdown, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
