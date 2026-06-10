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

    fs::write(output_root.join("styles.css"), render::stylesheet())?;
    fs::write(
        output_root.join("index.html"),
        render::render_index(&config, &posts),
    )?;

    for post in &posts {
        let post_dir = output_root.join("posts").join(&post.front_matter.slug);
        fs::create_dir_all(&post_dir)?;
        fs::write(post_dir.join("index.html"), render::render_post(&config, post))?;
    }

    fs::write(output_root.join("404.html"), render::render_404(&config))?;
    Ok(())
}
