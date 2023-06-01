/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let v = plain.to_lowercase().chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| if c.is_alphabetic() { (('a' as u8) + ('z' as u8) - (c as u8)) as char } else { c } )
        .collect::<String>().as_bytes().chunks(5)
        .map(|i| String::from_utf8(Vec::from(i)).unwrap())
        .collect::<Vec<_>>();
    let mut ret = String::new();
    for s in v {
        ret += &s;
        ret += " ";
    }
    ret.trim_end().to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| if c.is_alphabetic() { (('a' as u8) + ('z' as u8) - (c as u8)) as char } else { c } )
        .collect::<String>()
}
