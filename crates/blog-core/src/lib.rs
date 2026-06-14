pub mod content;
pub mod render;

use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn build_site(content_root: impl AsRef<Path>, output_root: impl AsRef<Path>) -> Result<()> {
    let content_root = content_root.as_ref();
    let output_root = output_root.as_ref();

    let config = content::load_site_config(content_root)?;
    let posts = content::load_posts(content_root)?;

    if output_root.exists() {
        fs::remove_dir_all(output_root)?;
    }
    fs::create_dir_all(output_root.join("posts"))?;

    let static_dir = content_root.join("static");
    if static_dir.exists() {
        copy_dir_all(&static_dir, output_root)?;
    }

    fs::write(output_root.join("styles.css"), render::stylesheet())?;
    fs::write(output_root.join("favicon.svg"), render::favicon_svg())?;
    fs::write(
        output_root.join("index.html"),
        render::render_index(&config, &posts),
    )?;

    let about_path = content_root.join("about.md");
    if about_path.exists() {
        if let Ok(about_page) = content::load_page_file(&about_path) {
            let about_dir = output_root.join("about");
            fs::create_dir_all(&about_dir)?;
            fs::write(about_dir.join("index.html"), render::render_page(&config, &about_page))?;
        }
    }

    for post in &posts {
        let post_dir = output_root.join("posts").join(&post.front_matter.slug);
        fs::create_dir_all(&post_dir)?;
        fs::write(post_dir.join("index.html"), render::render_post(&config, post))?;
    }

    fs::write(output_root.join("404.html"), render::render_404(&config))?;
    let search_index: Vec<_> = posts
        .iter()
        .filter(|p| !p.draft())
        .map(|p| {
            serde_json::json!({
                "title": p.title(),
                "slug": p.slug(),
                "description": p.description(),
                "body": p.body_plain_text,
            })
        })
        .collect();
    let search_index_json = serde_json::to_string(&search_index)?;
    fs::write(output_root.join("search_index.json"), search_index_json)?;

    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    if !src.exists() {
        return Ok(());
    }
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(entry.path(), dest_path)?;
        } else {
            fs::copy(entry.path(), dest_path)?;
        }
    }
    Ok(())
}
