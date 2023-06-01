/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn ctoi(c: char) -> i32 {
    ((c as u8) - ('a' as u8)) as i32
}
fn itoc(i: i32) -> char {
    (('a' as u8) + (i as u8)) as char
}
fn group_by_5(s: String) -> String {
    let v = s.as_bytes().chunks(5)
        .map(|i| String::from_utf8(Vec::from(i)).unwrap())
        .collect::<Vec<_>>();
    let mut ret = String::new();
    for s in v {
        ret += &s;
        ret += " ";
    }
    ret.trim_end().to_string()
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}
fn pow(mut n: i32, mut m: i32) -> i32 {
    n %= 26;
    let mut ret = 1;
    while m > 0 {
        if m & 1 == 1 { ret = ret * n % 26; }
        n = n * n % 26;
        m >>= 1;
    }
    ret
}
fn getinv(x: i32) -> i32 {
    // phi(26) == 11
    pow(x, 11)
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) > 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(group_by_5(plaintext.to_lowercase().chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| if c.is_alphabetic() { itoc((a * ctoi(c) + b) % 26) } else { c })
        .collect::<String>()))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) > 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(ciphertext.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| if c.is_alphabetic() { itoc((ctoi(c) - b % 26 + 26) * getinv(a) % 26) } else { c })
        .collect::<String>())
}
