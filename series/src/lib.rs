pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut ret = Vec::<String>::new();
    for i in 0..=digits.len() {
        if i + len > digits.len() { break; }
        ret.push(digits[i..i + len].to_string());
    }
    ret
}
