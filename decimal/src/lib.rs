use std::str::FromStr;

use num_bigint::BigInt;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Clone)]
pub struct Decimal {
    val: BigInt,
    offset: usize,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        if input.contains('.') == false {
            return Some(Decimal { val: BigInt::from_str(input).unwrap(), offset: 0 });
        } else {
            let pos = (input.len() - 1) - input.rfind('.').unwrap();
            let (s1, s2) = input.rsplit_once('.').unwrap();
            let s = s1.to_string() + s2;
            return Some(Decimal { val: BigInt::from_str(s.as_str()).unwrap(), offset: pos })
        }
    }
    fn relax(&self, new_offset: usize) -> Decimal {
        let mut ret = self.clone();
        let delta = new_offset - ret.offset;
        if delta > 0 {
            ret.val *= BigInt::from_str("10").unwrap().pow(delta as u32);
            ret.offset += delta;
        }
        ret
    }
    fn normalize(&self) -> Self {
        let mut ret = self.clone();
        while ret.val.clone() % 10 == BigInt::from_str("0").unwrap() {
            ret.val /= 10;
            ret.offset -= 1;
        }
        ret
    }
}

fn sync(u: &Decimal, v: &Decimal) -> (Decimal, Decimal) {
    let mx = std::cmp::max(u.offset, v.offset);
    (u.relax(mx), v.relax(mx))
}

impl std::cmp::PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        let (u, v) = sync(self, other);
        u.val == v.val
    }
}

impl std::cmp::PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (u, v) = sync(self, other);
        u.val.partial_cmp(&v.val)
    }
    fn lt(&self, other: &Self) -> bool {
        let (u, v) = sync(self, other);
        u.val < v.val
    }
    fn le(&self, other: &Self) -> bool {
        let (u, v) = sync(self, other);
        u.val <= v.val
    }
    fn gt(&self, other: &Self) -> bool {
        let (u, v) = sync(self, other);
        u.val > v.val
    }
    fn ge(&self, other: &Self) -> bool {
        let (u, v) = sync(self, other);
        u.val >= v.val
    }
}


impl std::ops::Add for Decimal {
    type Output = Decimal;
    fn add(self, rhs: Self) -> Self::Output {
        let (u, v) = sync(&self, &rhs);
        Decimal {
            val: u.val + v.val,
            offset: u.offset
        }.normalize()
    }
}

impl std::ops::Sub for Decimal {
    type Output = Decimal;
    fn sub(self, rhs: Self) -> Self::Output {
        let (u, v) = sync(&self, &rhs);
        Decimal {
            val: u.val - v.val,
            offset: u.offset
        }.normalize()
    }
}

impl std::ops::Mul for Decimal {
    type Output = Decimal;
    fn mul(self, rhs: Self) -> Self::Output {
        Decimal {
            val: self.val * rhs.val,
            offset: self.offset + rhs.offset
        }.normalize()
    }
}
