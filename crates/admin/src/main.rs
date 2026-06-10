use anyhow::{Context, Result};
use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
    routing::{get, post},
    Form, Json, Router,
};
use blog_core::content::{self, PostDraft};
use chrono::Local;
use serde::Deserialize;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    content_root: PathBuf,
    output_root: PathBuf,
}

#[derive(Debug, Deserialize)]
struct PostForm {
    title: String,
    slug: Option<String>,
    description: String,
    body_markdown: String,
    draft: Option<String>,
    date: Option<String>,
}

#[derive(Debug, Clone)]
struct PostFormData {
    title: String,
    slug: String,
    description: String,
    body_markdown: String,
    date: String,
    draft: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let state = AppState {
        content_root: PathBuf::from("content"),
        output_root: PathBuf::from("public"),
    };

    let app = Router::new()
        .route("/", get(dashboard))
        .route("/posts/new", get(new_post_form).post(create_post))
        .route("/posts/{slug}/edit", get(edit_post_form).post(update_post))
        .route("/posts/{slug}/delete", post(delete_post))
        .route("/build", post(rebuild_site))
        .route("/api/posts", get(list_posts))
        .with_state(state);

    let addr: SocketAddr = "127.0.0.1:8787".parse().expect("valid local address");
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("bind {addr}"))?;

    println!("Admin app listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn dashboard(State(state): State<AppState>) -> Html<String> {
    let posts = content::load_posts(&state.content_root).unwrap_or_default();
    Html(render_dashboard(&posts))
}

async fn new_post_form() -> Html<String> {
    Html(render_post_form(
        "New Post",
        "/posts/new",
        "Create Post",
        &PostFormData {
            title: String::new(),
            slug: String::new(),
            description: String::new(),
            body_markdown: String::new(),
            date: today_string(),
            draft: false,
        },
    ))
}

async fn edit_post_form(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Html<String>, (axum::http::StatusCode, String)> {
    let post = content::load_post_by_slug(&state.content_root, &slug)
        .map_err(internal_error)?
        .ok_or_else(|| (axum::http::StatusCode::NOT_FOUND, format!("post {slug} not found")))?;

    let action = format!("/posts/{}/edit", escape_html(&slug));
    Ok(Html(render_post_form(
        &format!("Edit: {}", post.title()),
        &action,
        "Save Changes",
        &PostFormData {
            title: post.title().to_string(),
            slug: post.slug().to_string(),
            description: post.description().to_string(),
            body_markdown: post.body_markdown.clone(),
            date: post.date().to_string(),
            draft: post.draft(),
        },
    )))
}

async fn create_post(
    State(state): State<AppState>,
    Form(input): Form<PostForm>,
) -> Result<Redirect, (axum::http::StatusCode, String)> {
    save_post_form(&state.content_root, input, None).map_err(internal_error)?;
    Ok(Redirect::to("/"))
}

async fn update_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Form(input): Form<PostForm>,
) -> Result<Redirect, (axum::http::StatusCode, String)> {
    let final_slug = save_post_form(&state.content_root, input, Some(&slug)).map_err(internal_error)?;
    if final_slug != slug {
        content::delete_post(&state.content_root, &slug).map_err(internal_error)?;
    }
    Ok(Redirect::to("/"))
}

async fn delete_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Redirect, (axum::http::StatusCode, String)> {
    content::delete_post(&state.content_root, &slug).map_err(internal_error)?;
    Ok(Redirect::to("/"))
}

async fn rebuild_site(State(state): State<AppState>) -> Result<Redirect, (axum::http::StatusCode, String)> {
    blog_core::build_site(&state.content_root, &state.output_root).map_err(internal_error)?;
    Ok(Redirect::to("/"))
}

async fn list_posts(State(state): State<AppState>) -> Json<Vec<content::Post>> {
    let posts = content::load_posts(&state.content_root).unwrap_or_default();
    Json(posts)
}

fn save_post_form(
    content_root: &std::path::Path,
    input: PostForm,
    existing_slug: Option<&str>,
) -> Result<String> {
    let slug = normalized_slug(&input, existing_slug);
    let date = input
        .date
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(today_string);
    let draft = PostDraft {
        title: input.title,
        date,
        slug: slug.clone(),
        description: input.description,
        draft: input.draft.is_some(),
        body_markdown: input.body_markdown,
    };

    content::save_post(content_root, &draft)?;
    Ok(slug)
}

fn normalized_slug(input: &PostForm, existing_slug: Option<&str>) -> String {
    let slug = input.slug.as_deref().unwrap_or_default().trim();
    if !slug.is_empty() {
        return slug.to_string();
    }

    if let Some(existing_slug) = existing_slug {
        return existing_slug.to_string();
    }

    content::slugify(&input.title)
}

fn render_dashboard(posts: &[content::Post]) -> String {
    let mut rows = String::new();

    for post in posts {
        let status = if post.draft() { "Draft" } else { "Published" };
        rows.push_str(&format!(
            r#"<article class="card">
  <div class="card-top">
    <div>
      <h3>{title}</h3>
      <p class="meta">{date} · <code>{slug}</code> · <span class="status">{status}</span></p>
      <p>{description}</p>
    </div>
    <div class="actions">
      <a class="button" href="/posts/{slug}/edit">Edit</a>
      <form method="post" action="/posts/{slug}/delete" onsubmit="return confirm('Delete this post?');">
        <button class="button danger" type="submit">Delete</button>
      </form>
    </div>
  </div>
</article>"#,
            title = escape_html(post.title()),
            date = escape_html(post.date()),
            slug = escape_html(post.slug()),
            status = status,
            description = escape_html(post.description()),
        ));
    }

    render_page(
        "r-blog admin",
        &format!(
            r#"<section class="hero">
  <div>
    <span class="badge">Local-only admin</span>
    <h1>Blog control panel</h1>
    <p>Manage Markdown posts on your machine, then rebuild the static site when you are ready to push to GitHub.</p>
  </div>
  <div class="hero-actions">
    <a class="button primary" href="/posts/new">New Post</a>
    <form method="post" action="/build">
      <button class="button" type="submit">Rebuild Site</button>
    </form>
  </div>
</section>
<section class="panel">
  <h2>Posts ({count})</h2>
  {rows}
</section>"#,
            count = posts.len(),
            rows = rows,
        ),
    )
}

fn render_post_form(title: &str, action: &str, submit_label: &str, data: &PostFormData) -> String {
    render_page(
        title,
        &format!(
            r#"<section class="panel form-panel">
  <div class="panel-header">
    <div>
      <span class="badge">Markdown editor</span>
      <h1>{title}</h1>
    </div>
    <a class="button" href="/">Back</a>
  </div>
  <form method="post" action="{action}" class="post-form">
    <label>
      <span>Title</span>
      <input name="title" value="{title_value}" required>
    </label>
    <label>
      <span>Slug</span>
      <input name="slug" value="{slug_value}" placeholder="optional if you want auto-generated slug">
    </label>
    <label>
      <span>Date</span>
      <input name="date" value="{date_value}" placeholder="YYYY-MM-DD">
    </label>
    <label>
      <span>Description</span>
      <input name="description" value="{description_value}" required>
    </label>
    <label class="checkbox-row">
      <input type="checkbox" name="draft" value="true" {draft_checked}>
      <span>Save as draft</span>
    </label>
    <label>
      <span>Body</span>
      <textarea name="body_markdown" rows="18" required>{body_value}</textarea>
    </label>
    <div class="form-actions">
      <button class="button primary" type="submit">{submit_label}</button>
      <a class="button" href="/">Cancel</a>
    </div>
  </form>
</section>"#,
            title = escape_html(title),
            action = escape_html(action),
            submit_label = escape_html(submit_label),
            title_value = escape_html(&data.title),
            slug_value = escape_html(&data.slug),
            date_value = escape_html(&data.date),
            description_value = escape_html(&data.description),
            draft_checked = if data.draft { "checked" } else { "" },
            body_value = escape_html(&data.body_markdown),
        ),
    )
}

fn render_page(title: &str, body: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{title}</title>
  <style>
    :root {{
      color-scheme: light;
      --bg: #f4efe7;
      --panel: rgba(255, 253, 248, 0.9);
      --panel-solid: #fffdf8;
      --text: #1f2328;
      --muted: #5c6670;
      --border: #ded4c7;
      --accent: #0f766e;
      --accent-strong: #115e59;
      --danger: #b42318;
      --shadow: 0 20px 60px rgba(31, 35, 40, 0.08);
    }}
    * {{ box-sizing: border-box; }}
    body {{ margin: 0; font-family: Inter, system-ui, sans-serif; background:
      radial-gradient(circle at top left, rgba(15,118,110,0.1), transparent 28%),
      linear-gradient(180deg, #f9f6f0 0%, var(--bg) 100%); color: var(--text); }}
    a {{ color: inherit; text-decoration: none; }}
    main {{ max-width: 1100px; margin: 0 auto; padding: 32px 20px 56px; }}
    .hero, .panel, .card, .form-panel {{
      background: var(--panel);
      backdrop-filter: blur(10px);
      border: 1px solid var(--border);
      border-radius: 24px;
      box-shadow: var(--shadow);
    }}
    .hero {{ display: flex; justify-content: space-between; gap: 20px; padding: 28px; margin-bottom: 24px; align-items: center; }}
    .hero h1, .panel h2, .panel-header h1, .card h3 {{ margin: 0; }}
    .hero p, .card p, .meta {{ color: var(--muted); }}
    .badge {{ display: inline-flex; padding: 4px 10px; border-radius: 999px; background: rgba(15,118,110,0.12); color: var(--accent); font-size: 0.8rem; font-weight: 700; margin-bottom: 12px; }}
    .hero-actions, .actions, .form-actions {{ display: flex; gap: 10px; align-items: center; flex-wrap: wrap; }}
    .panel {{ padding: 24px; margin-bottom: 20px; }}
    .card {{ padding: 18px 20px; margin-top: 16px; }}
    .card-top {{ display: flex; justify-content: space-between; gap: 16px; align-items: flex-start; }}
    .meta code, code {{ background: rgba(15,118,110,0.1); padding: 0.15rem 0.35rem; border-radius: 6px; }}
    .status {{ color: var(--accent-strong); font-weight: 700; }}
    .button {{ display: inline-flex; align-items: center; justify-content: center; gap: 6px; padding: 10px 16px; border-radius: 12px; border: 1px solid var(--border); background: var(--panel-solid); cursor: pointer; font: inherit; }}
    .button:hover {{ border-color: #c8baab; }}
    .button.primary {{ background: var(--accent); color: #fff; border-color: var(--accent); }}
    .button.primary:hover {{ background: var(--accent-strong); }}
    .button.danger {{ color: var(--danger); }}
    .post-form {{ display: grid; gap: 16px; margin-top: 18px; }}
    label {{ display: grid; gap: 8px; font-weight: 600; }}
    input, textarea {{ width: 100%; border-radius: 14px; border: 1px solid var(--border); background: var(--panel-solid); padding: 12px 14px; font: inherit; color: var(--text); }}
    textarea {{ resize: vertical; min-height: 260px; font-family: ui-monospace, SFMono-Regular, Consolas, monospace; line-height: 1.6; }}
    .checkbox-row {{ display: flex; align-items: center; gap: 10px; font-weight: 500; }}
    .checkbox-row input {{ width: auto; }}
    .panel-header {{ display: flex; justify-content: space-between; gap: 16px; align-items: center; }}
    @media (max-width: 760px) {{
      main {{ padding: 16px 12px 36px; }}
      .hero, .panel {{ padding: 18px; }}
      .card-top, .panel-header, .hero {{ flex-direction: column; align-items: stretch; }}
      .hero-actions, .actions {{ width: 100%; }}
      .button {{ width: 100%; }}
    }}
  </style>
</head>
<body>
  <main>
    {body}
  </main>
</body>
</html>"#,
        title = escape_html(title),
        body = body,
    )
}

fn today_string() -> String {
    Local::now().date_naive().format("%F").to_string()
}

fn internal_error(error: anyhow::Error) -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
