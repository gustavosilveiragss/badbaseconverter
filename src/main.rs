/*use clap::Parser;

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
} */

mod bintodec;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    bin: String,
}

fn get_binary() -> String {
    let args = Cli::parse();

    let bin = &args.bin.trim().to_string();

    // check if input consists only of 1 and 0
    if !bin.chars().all(|x| "01".contains(x)) {
        println!("INVALID CHARACTERS. TRY AGAIN:");
        get_binary();
    }

    return bin.clone();
}

use anyhow::Result;
fn main() -> Result<()> {
    let bin = get_binary();

    bintodec::bi_to_dec(bin);

    Ok(())
}

