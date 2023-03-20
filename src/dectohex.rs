use crate::utils::reverse_new;

pub fn dec_to_hex(dec: String) {
    let mut hex = String::from("");

    let mut result: f64 = dec.parse().expect("DECIMAL INPUTTED IS NOT A NUMBER");

    while result > 0.0 {

        let remainder = (result % 16.0) as u64;

        let new_char: char = match remainder {
            0..=9 => remainder.to_string().chars().nth(0).unwrap(),
            10 => 'A',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            14 => 'E',
            15 => 'F',
            _ => panic!("SOMETHING WENT WRONG IN DIVIDING")
        };

        hex.push(new_char);

        result = (result / 16.0).trunc();
    }

    println!("{}", reverse_new(hex));
}