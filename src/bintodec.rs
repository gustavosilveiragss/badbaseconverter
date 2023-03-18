pub fn bi_to_dec(bin: String) {
    let mut sum: usize = 0;

    for (char, i) in bin.chars().zip(0u32..) {
        let inverted_pos: u32 = (bin.chars().count() - (i as usize + 1)) as u32;

        if char == '1' {
            sum += 2usize.pow(inverted_pos);
        }
    }

    println!("{sum}");
}