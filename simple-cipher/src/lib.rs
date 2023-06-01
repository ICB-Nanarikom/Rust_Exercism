use rand::{distributions::Uniform, prelude::Distribution};

fn ctoi(c: char) -> i32 {
    ((c as u8) - ('a' as u8)) as i32
}
fn itoc(i: i32) -> char {
    (('a' as u8) + (i as u8)) as char
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || s.is_empty() {
        return None;
    }
    if key != key.to_lowercase() || s != s.to_lowercase() {
        return None;
    }
    if key.chars().any(|c| c.is_alphabetic() == false) || s.chars().any(|c| c.is_alphabetic() == false) {
        return None;
    }

    let key = key.repeat(10);
    let mut it = key.chars();
    Some(s.chars().map(|c| itoc((ctoi(c) + ctoi(it.next().unwrap())) % 26) ).collect::<String>())
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || s.is_empty() {
        return None;
    }
    if key != key.to_lowercase() || s != s.to_lowercase() {
        return None;
    }
    if key.chars().any(|c| c.is_alphabetic() == false) || s.chars().any(|c| c.is_alphabetic() == false) {
        return None;
    }

    let key = key.repeat(10);
    let mut it = key.chars();
    Some(s.chars().map(|c| itoc((ctoi(c) + 26 - ctoi(it.next().unwrap())) % 26) ).collect::<String>())
}

pub fn encode_random(s: &str) -> (String, String) {
    use rand::thread_rng;
    let rng = thread_rng();
    let key = Uniform::from('a'..='z').sample_iter(rng).take(100).collect::<String>();
    let res = encode(key.as_str(), s).unwrap();
    (key, res)
}
