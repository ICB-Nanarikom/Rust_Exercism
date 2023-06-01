use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if "AGCT".contains(nucleotide) == false { return Err('X'); }
    if dna.chars().all(|c| "AGCT".contains(c)) { Ok(dna.chars().filter(|c| *c == nucleotide ).count()) } else { Err('X') }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut ret = HashMap::<char, usize>::new();
    for c in "AGCT".chars() {
        match count(c, dna) {
            Ok(cnt) => { ret.insert(c, cnt); },
            Err(e) => { return Err(e); },
        }
    }
    Ok(ret)
}
