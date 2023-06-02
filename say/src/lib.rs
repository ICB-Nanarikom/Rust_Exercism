const S1: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

fn ad_hoc(n: u64) -> Option<String> {
    match n {
        0 => Some("zero".to_string()),
        1 => Some("one".to_string()),
        2 => Some("two".to_string()),
        3 => Some("three".to_string()),
        4 => Some("four".to_string()),
        5 => Some("five".to_string()),
        6 => Some("six".to_string()),
        7 => Some("seven".to_string()),
        8 => Some("eight".to_string()),
        9 => Some("nine".to_string()),
        10 => Some("ten".to_string()),
        11 => Some("eleven".to_string()),
        12 => Some("twelve".to_string()),
        13 => Some("thirteen".to_string()),
        15 => Some("fifteen".to_string()),
        18 => Some("eighteen".to_string()),
        20 => Some("twenty".to_string()),
        30 => Some("thirty".to_string()),
        40 => Some("forty".to_string()),
        50 => Some("fifty".to_string()),
        80 => Some("eighty".to_string()),
        _ => None,
    }
}

fn single(n: u64) -> Option<String> {
    match ad_hoc(n) {
        Some(s) => Some(s),
        None => match n {
            n if 14 <= n && n <= 19 => Some(ad_hoc(n % 10).unwrap() + "teen"),
            n if 20 <= n && n <= 100 && n % 10 == 0 => Some(ad_hoc(n / 10).unwrap() + "ty"),
            _ => None,
        },
    }
}

fn double(n: u64) -> String {
    match single(n) {
        Some(s) => s,
        None => single(n / 10 * 10).unwrap() + "-" + single(n % 10).unwrap().as_str(),
    }
}

fn triple(n: u64) -> String {
    let s1 = if n >= 100 { Some(single(n / 100).unwrap() + " hundred") } else { None };
    let s2 = if n % 100 > 0 { Some(double(n % 100)) } else { None };
    match (s1, s2) {
        (Some(s1), Some(s2)) => s1 + " " + s2.as_str(),
        (Some(s), None) => s,
        (None, Some(s)) => s,
        _ => String::new(),
    }
}

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let mut v = Vec::new();
    let mut it = S1.into_iter();
    while n > 0 {
        v.push((n % 1000, it.next().unwrap()));
        n /= 1000;
    }

    v.into_iter().rev().map(|(n, it)| {
        if n > 0 { triple(n) + " " + it + " " } else { String::new() }
    }).collect::<String>().trim_end().to_string()
}
