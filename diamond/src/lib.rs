pub fn get_diamond(c: char) -> Vec<String> {
    let n = ((c as u8) - ('A' as u8)) as usize;
    let mut ret = Vec::<String>::new();
    let mut tmp = ('A'..=c).collect::<String>();
    tmp.push_str(('A'..c).rev().collect::<String>().as_str());
    for i in tmp.chars() {
        let mut s = String::from(" ").repeat(2 * n + 1);
        let offset = ((i as u8) - ('A' as u8)) as usize;
        s.replace_range((n - offset)..=(n - offset), i.to_string().as_str());
        s.replace_range((n + offset)..=(n + offset), i.to_string().as_str());
        ret.push(s);
    }
    ret
}