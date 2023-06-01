/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.chars().filter(|c| *c != '-').collect::<String>();
    if isbn.len() != 10 {
        return false;
    }
    if isbn.chars().any(|c| c.is_digit(10) == false && c != 'X') {
        return false;
    }

    let mut sum: u32 = 0;
    for (i, c) in isbn.chars().enumerate() {
        if i < 9 && c == 'X' { return false; }
        sum += ((10 - i) as u32) * (if c != 'X' { c.to_digit(10).unwrap() } else { 10 });
    }
    sum % 11 == 0
}
