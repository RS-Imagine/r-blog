use anyhow::Result;
use blog_core::build_site;
use std::env;

fn main() -> Result<()> {
    let command = env::args().nth(1).unwrap_or_else(|| "build".to_string());

    match command.as_str() {
        "build" => {
            build_site("content", "public")?;
            println!("Built static site into public/");
            Ok(())
        }
        other => {
            eprintln!("Unknown command: {other}");
            eprintln!("Usage: cargo run -p sitegen -- build");
            std::process::exit(1);
        }
    }
}
