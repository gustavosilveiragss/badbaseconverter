mod bintodec;
mod dectobin;
mod utils;
mod dectohex;

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
    let dec = &args.num.trim().to_string();

    dectobin::dec_to_bi(dec.clone());
}

fn decimal_to_hexadecimal(args: Cli) {
    let dec = &args.num.trim().to_string();

    dectohex::dec_to_hex(dec.clone());
}

use anyhow::Result;
fn main() -> Result<()> {
    let args = Cli::parse();

    match args.pattern.as_str() {
        "bitodec" | "btd" | "bintodec" | "binarytodecimal" => binary_to_decimal(args),
        "dectobi" | "dtb" | "dectobin" | "decimaltobinary" => decimal_to_binary(args),
        "dectohex" | "dth" | "dechex" | "decimaltohexadecimal" => decimal_to_hexadecimal(args),
        &_ => println!("MUST INPUT CONVERSION PATTERN MATCHING OPTIONS")
    }

    Ok(())
}

