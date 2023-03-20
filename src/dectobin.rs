use crate::utils::reverse_new;

pub fn dec_to_bi(dec: String) {
    let mut bin = String::from("");

    let mut result: f64 = dec.parse().expect("DECIMAL INPUTTED IS NOT A NUMBER");

    while result > 0.0 {
        result = result / 2.0;

        if result.fract() == 0.0 {
            bin.push('0');
        } else {
            bin.push('1');
        }

        result = result.trunc();
    }

    println!("{}", reverse_new(bin));
}