mod bintodec;
mod dectobin;
mod dectohex;
mod hextodec;
mod utils;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    num: String,
}

use anyhow::Result;
fn main() -> Result<()> {
    let args = Cli::parse();

    let num = &args.num.trim().to_string();

    match args.pattern.as_str() {
        "bitodec" | "btd" | "bintodec" | "binarytodecimal" => {
            // check if input consists only of 1 and 0
            if !num.chars().all(|x| "01".contains(x)) {
                panic!("INVALID CHARACTERS. TRY AGAIN");
            }

            bintodec::bi_to_dec(num.clone());
            ()
        }
        "dectobi" | "dtb" | "dectobin" | "decimaltobinary" => {
            dectobin::dec_to_bi(num.clone());
            ()
        }
        "dectohex" | "dth" | "dechex" | "decimaltohexadecimal" => {
            dectohex::dec_to_hex(num.clone());
            ()
        }
        "hextodec" | "htd" | "hexdec" | "hexadecimaltodecimal" => {
            hextodec::hex_to_dec(num.clone());
            ()
        }
        "bintohex" | "bth" | "binhex" | "binarytohexadecimal" => {
            let dec = bintodec::bi_to_dec(num.clone());

            dectohex::dec_to_hex(dec);
            ()
        }
        "hextobin" | "htb" | "hexbin" | "hexadecimaltobinary" => {
            let dec = hextodec::hex_to_dec(num.clone());

            dectobin::dec_to_bi(dec);
            ()
        }
        &_ => panic!("MUST INPUT CONVERSION PATTERN MATCHING OPTIONS"),
    }

    Ok(())
}
