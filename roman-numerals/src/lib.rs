use std::fmt::{Display, Formatter, Result};

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut s = String::new();
        s += digit(num / 1000, "M", "", "").as_str();
        s += digit(num / 100 % 10, "C", "D", "M").as_str();
        s += digit(num / 10 % 10, "X", "L", "C").as_str();
        s += digit(num % 10, "I", "V", "X").as_str();
        Roman(s)
    }
}

fn digit (d: u32, c1: &str, c5: &str, c10: &str) -> String {
    match d as usize {
        i if 1 <= i && i <= 3   => c1.repeat(i),
        i if i == 4             => c1.to_string() + c5,
        i if i == 5             => c5.to_string(),
        i if 6 <= i && i <= 8   => c5.to_string() + c1.repeat(i - 5).as_str(),
        i if i == 9             => c1.to_string() + c10,
        _ => String::new(),
    }
}