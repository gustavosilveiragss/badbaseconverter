pub fn bi_to_dec(bin: String) {
    let bytes = bin.as_bytes();

    let mut sum: usize = 0;

    for (i, &char) in bytes.iter().enumerate() {
        let inverted_pos = bin.chars().count() - (i + 1);

        if char == b'1' {
            sum += 2usize.pow(inverted_pos as u32);
        }
    }

    println!("{sum}");
}
