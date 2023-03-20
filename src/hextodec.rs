pub fn hex_to_dec(hex: String) {
    let bytes = hex.as_bytes();

    let mut sum: usize = 0;

    for (i, &char) in bytes.iter().enumerate() {
        let inverted_pos = hex.chars().count() - (i + 1);

        let char_val: usize = match char {
            b'A' => 10,
            b'B' => 11,
            b'C' => 12,
            b'D' => 13,
            b'E' => 14,
            b'F' => 15,
            _ => char as usize - 48
        };

        sum += char_val * 16usize.pow(inverted_pos as u32);
    }

    println!("{sum}");
}