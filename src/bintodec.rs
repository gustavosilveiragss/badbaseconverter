pub fn bi_to_dec(bin: String) -> String {
    let sum = bin.as_bytes()
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &b)| {
            let inverted_pos = bin.len() - i - 1;
            acc + if b == b'1' { 2usize.pow(inverted_pos as u32) } else { 0 }
        });

    println!("the binary \"{bin}\" in decimal is: {sum}");

    sum.to_string()
}
