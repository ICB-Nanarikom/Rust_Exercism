/// Check a Luhn checksum.

pub fn is_valid(code: &str) -> bool {
    let code = code.chars().filter(|c| *c != ' ').rev().collect::<String>();

    if code.len() <= 1 {
        return false;
    }
    for c in code.chars() {
        if c.is_ascii_digit() == false {
            return false;
        }
    }

    let mut sum = 0;
    for (i, c) in code.chars().enumerate() {
        let mut x = c.to_digit(10).unwrap();
        if i % 2 == 1 {
            x *= 2;
        }
        sum += if x > 9 {x - 9} else {x};
    }

    sum % 10 == 0
}
