pub fn encode(source: &str) -> String {
    let mut s= source.to_string() + "$".to_string().as_str();
    let mut ret = String::new();
    while s.is_empty() == false {
        let fir = s.chars().nth(0).unwrap();
        match s.find(|c| c != fir) {
            Some(pos) => {
                if pos > 1 { ret += pos.to_string().as_str(); }
                ret += fir.to_string().as_str();
                s = s.split_at(pos).1.to_string();
            },
            None => {
                break;
            },
        }
    }
    ret
}

pub fn decode(source: &str) -> String {
    let mut s = source.to_string();
    let mut ret = String::new();
    while s.is_empty() == false {
        let pos = s.find(|c: char| c.is_numeric() == false).unwrap();
        let (cnt, suf) = s.split_at(pos);
        let (c, suf) = suf.split_at(1);
        ret += c.repeat(cnt.parse::<usize>().unwrap_or(1)).as_str();
        s = suf.to_string();
    }
    ret
}
