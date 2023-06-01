fn ctoi(c: char, up: bool) -> i32 {
    ((c as u8) - (if up { 'A' } else { 'a' } as u8)) as i32
}
fn itoc(i: i32, up: bool) -> char {
    ((if up { 'A' } else { 'a' } as u8) + (i as u8)) as char
}

pub fn rotate(input: &str, key: i8) -> String {
    let key = ((key % 26 + 26) % 26) as i32;
    input.chars().map(|c| if c.is_alphabetic() {
        let up = c.is_uppercase();
        return itoc((ctoi(c, up) + key) % 26, up);
    } else {
        return c;
    }).collect::<String>()
}
