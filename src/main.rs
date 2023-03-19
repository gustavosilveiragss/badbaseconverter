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
    pattern: String,
    num: String,
}

fn binary_to_decimal(args: Cli) {
    let bin = &args.num.trim().to_string();

    // check if input consists only of 1 and 0
    if !bin.chars().all(|x| "01".contains(x)) {
        println!("INVALID CHARACTERS. TRY AGAIN:");
        binary_to_decimal(args);
    }

    bintodec::bi_to_dec(bin.clone());
}

fn decimal_to_binary(args: Cli) {
    
}

use anyhow::Result;
fn main() -> Result<()> {
    let args = Cli::parse();

    match args.pattern.as_str() {
        "bitodec" | "btd" | "bintodec" | "binarytodecimal" => binary_to_decimal(args),
        "dectobi" | "dtb" | "dectobin" | "decimaltobinary" => decimal_to_binary(args),
        &_ => println!("MUST INPUT CONVERSION PATTERN MATCHING OPTIONS")
    }

    Ok(())
}

