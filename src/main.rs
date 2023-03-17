use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path = "test.txt";

    let content =
        std::fs::read_to_string(path).with_context(|| format!("could not read file `{}`", path))?;

    println!("file content: {}", content);

    Ok(())
}
