use pulldown_cmark::{html, Options, Parser};

use crate::math::{preprocess_math, render_math_expr};

/// Converts Markdown source to an HTML string.
///
/// The pipeline is:
/// 1. Pre-process math fences (`$`, `$$`) → replace with placeholders.
/// 2. Run pulldown-cmark with all extensions enabled.
/// 3. Substitute placeholders with KaTeX-rendered HTML.
pub(crate) fn markdown_to_html(markdown: &str) -> String {
    let (processed, math_segments) = preprocess_math(markdown);
    let parser = Parser::new_ext(&processed, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    for segment in math_segments {
        let rendered = render_math_expr(&segment.math, segment.display_mode);

        if segment.display_mode {
            // Replace the `<p><placeholder></p>` wrapper that cmark generates
            // for block-level elements with a proper `<div class="math-block">`.
            let wrapped = format!("<p>{}</p>", segment.placeholder);
            if html_output.contains(&wrapped) {
                html_output = html_output.replace(
                    &wrapped,
                    &format!(r#"<div class="math-block">{rendered}</div>"#),
                );
            } else {
                html_output = html_output.replace(&segment.placeholder, &rendered);
            }
        } else {
            html_output = html_output.replace(&segment.placeholder, &rendered);
        }
    }

    html_output
}

/// Strips all Markdown/math syntax from `markdown` and returns plain text.
///
/// Used to build the client-side search index (`search_index.json`).
pub(crate) fn markdown_to_plain_text(markdown: &str) -> String {
    let (processed, _) = preprocess_math(markdown);
    let parser = Parser::new_ext(&processed, Options::all());
    let mut text = String::new();

    for event in parser {
        match event {
            pulldown_cmark::Event::Text(t) | pulldown_cmark::Event::Code(t) => {
                text.push_str(&t);
                text.push(' ');
            }
            pulldown_cmark::Event::SoftBreak | pulldown_cmark::Event::HardBreak => {
                text.push(' ');
            }
            _ => {}
        }
    }

    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
