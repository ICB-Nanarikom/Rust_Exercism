/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let s1 = value.to_string();
        let s2 = s1.chars().rev().collect::<String>();
        if s1 == s2 { Some(Palindrome(value)) } else { None }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut mn = Palindrome::new(999999999);
    let mut mx = Palindrome::new(0);
    let mut succ = false;
    for i in min..=max {
        for j in i..=max {
            let tmp = Palindrome::new(i * j);
            match tmp {
                Some(_) => {
                    succ = true;
                    mn = std::cmp::min(mn, tmp);
                    mx = std::cmp::max(mx, tmp);
                },
                None => {},
            }
        }
    }
    if succ { Some((mn.unwrap(), mx.unwrap())) } else { None }
}
