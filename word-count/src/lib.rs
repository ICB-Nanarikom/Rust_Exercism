use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut ret = HashMap::<String, u32>::new();
    for word in words.to_lowercase().split(|c: char| c.is_alphanumeric() == false && c != '\'').map(|word| word.trim_matches('\'')) {
        if word == "" {
            continue;
        }
        match ret.get_mut(word) {
            Some(v) => { *v += 1; },
            None => { ret.insert(word.to_string(), 1); },
        }
    }
    ret
}
