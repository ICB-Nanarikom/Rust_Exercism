/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.to_lowercase().chars().fold(0, |sum, c| sum + match c {
        c if "aeioulnrst".contains(c) => 1,
        c if "dg".contains(c) => 2,
        c if "bcmp".contains(c) => 3,
        c if "fhvwy".contains(c) => 4,
        c if "k".contains(c) => 5,
        c if "jx".contains(c) => 8,
        c if "qz".contains(c) => 10,
        _ => 0,
    })
}
