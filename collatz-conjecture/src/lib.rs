pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut cnt: u64 = 0;
    while n > 1 {
        match n {
            0 => { return None; }
            1 => { return Some(cnt); }
            tmp if tmp % 2 == 0 => { n /= 2; }
            _ /* if tmp % 2 == 1 */ => { n = n.checked_mul(3)?.checked_add(1)?; }
        }
        cnt += 1;
    }
    Some(cnt)
}
