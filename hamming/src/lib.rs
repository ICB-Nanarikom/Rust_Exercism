/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let mut ret = 0;
    let mut it1 = s1.chars();
    let mut it2 = s2.chars();
    loop {
        let c1 = it1.next().unwrap_or('$');
        let c2 = it2.next().unwrap_or('$');
        match c1 {
            '$' => { break Some(ret as usize); },
            _ => { ret += if c1 != c2 { 1 } else { 0 }; }
        }
    }
}
