#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn factors_sum(num: u64) -> u64 {
    if num == 1 {
        return 0;
    }

    let mut ret = 1;
    for d in 2..=num {
        if d * d > num {
            break;
        }
        if num % d == 0 {
            ret += d;
            if d != num / d { ret += num / d; }
        }
    }
    ret
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    match (factors_sum(num), num) {
        (a, b) if a > b => { Some(Classification::Abundant) },
        (a, b) if a == b => { Some(Classification::Perfect) },
        _ /* if a < b */ => { Some(Classification::Deficient) },
    }
}
