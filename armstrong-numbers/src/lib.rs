pub fn is_armstrong_number(num: u64) -> bool {
    let str = num.to_string();
    let mut sum: u64 = 0;
    for c in str.chars() {
        sum += c.to_digit(10).unwrap().pow(str.len() as u32) as u64;
    }
    sum == num
}
