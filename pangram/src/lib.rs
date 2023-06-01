/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let sentence = sentence.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect::<String>();
    if ('a'..='z').any(|c| sentence.contains(c) == false) {
        return false;
    }
    true
}
