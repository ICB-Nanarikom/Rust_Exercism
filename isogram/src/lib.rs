pub fn check(candidate: &str) -> bool {
    let mut s = candidate.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect::<Vec<_>>();
    let n = s.len();
    s.sort();
    s.dedup();
    s.len() == n
}
