use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut ret = HashSet::<&str>::new();

    let mut word_vec: Vec<char> = word.to_lowercase().chars().collect();
    word_vec.sort();
    for i in possible_anagrams {
        let mut i_vec: Vec<char> = i.to_lowercase().chars().collect();
        i_vec.sort();
        if i.to_lowercase() == word.to_lowercase() {
            continue;
        }
        if i_vec == word_vec {
            ret.insert(i);
        }
    }
    ret
}
