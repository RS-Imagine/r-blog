pub mod content;
pub mod render;

use anyhow::Result;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn build_site(content_root: impl AsRef<Path>, output_root: impl AsRef<Path>) -> Result<()> {
    let content_root = content_root.as_ref();
    let output_root = output_root.as_ref();

    let config = content::load_site_config(content_root)?;
    let posts = content::load_posts(content_root)?;

    fs::create_dir_all(output_root.join("posts"))?;

    let static_dir = content_root.join("static");
    if static_dir.exists() {
        copy_dir_all(&static_dir, output_root)?;
    }

    write_if_changed(output_root.join("styles.css"), render::stylesheet())?;
    write_if_changed(output_root.join("favicon.svg"), render::favicon_svg())?;
    write_if_changed(
        output_root.join("index.html"),
        render::render_index(&config, &posts),
    )?;

    let about_path = content_root.join("about.md");
    let about_dir = output_root.join("about");
    if about_path.exists() {
        if let Ok(about_page) = content::load_page_file(&about_path) {
            fs::create_dir_all(&about_dir)?;
            write_if_changed(about_dir.join("index.html"), render::render_page(&config, &about_page))?;
        }
    } else if about_dir.exists() {
        fs::remove_dir_all(&about_dir)?;
    }

    let posts_dir = output_root.join("posts");
    let valid_slugs: HashSet<_> = posts.iter().map(|p| p.slug()).collect();
    if posts_dir.exists() {
        for entry in fs::read_dir(&posts_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(slug) = path.file_name().and_then(|n| n.to_str()) {
                    if !valid_slugs.contains(slug) {
                        fs::remove_dir_all(&path)?;
                    }
                }
            }
        }
    }

    for post in &posts {
        let post_dir = output_root.join("posts").join(&post.front_matter.slug);
        fs::create_dir_all(&post_dir)?;
        write_if_changed(post_dir.join("index.html"), render::render_post(&config, post))?;
    }

    write_if_changed(output_root.join("404.html"), render::render_404(&config))?;
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
    write_if_changed(output_root.join("search_index.json"), search_index_json)?;

    Ok(())
}

fn write_if_changed(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> Result<()> {
    let path = path.as_ref();
    let content = content.as_ref();
    if path.exists() {
        if let Ok(existing) = fs::read(path) {
            if existing == content {
                return Ok(());
            }
        }
    }
    fs::write(path, content)?;
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
            if dest_path.exists() {
                let src_meta = entry.metadata()?;
                let dst_meta = fs::metadata(&dest_path)?;
                if src_meta.len() == dst_meta.len() {
                    if let (Ok(src_time), Ok(dst_time)) = (src_meta.modified(), dst_meta.modified()) {
                        if dst_time >= src_time {
                            continue;
                        }
                    }
                }
            }
            fs::copy(entry.path(), dest_path)?;
        }
    }
    Ok(())
}
