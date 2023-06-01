fn transpose(mat: &[&[u8]], r: usize, c: usize) -> Vec<String> {
    let mut ret = Vec::<String>::new();
    for i in 0..c {
        let mut cur = Vec::<u8>::new();
        for j in 0..r {
            cur.push(if i < mat[j].len() { mat[j][i] } else { ' ' as u8 });
        }
        ret.push(String::from_utf8(cur).unwrap());
    }
    ret
}

pub fn encrypt(input: &str) -> String {
    if input == "" {
        return String::new();
    }

    let input = input.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    let mut c = (input.len() as f32).sqrt() as usize;
    if c.pow(2) < input.len() { c += 1; }
    let r = (input.len() - 1) / c + 1;
    let mat = transpose(input.as_bytes().chunks(c).collect::<Vec<_>>().as_slice(), r, c);
    let mut ret = String::new();
    for (i, s) in mat.into_iter().enumerate() {
        ret += &s;
        if i < c - 1 { ret += " "; }
    }
    ret
}
