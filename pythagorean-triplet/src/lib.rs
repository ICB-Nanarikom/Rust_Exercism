use std::collections::HashSet;

fn solve(a: i64, b: i64, c: i64) -> i64 {
    let delta = b.pow(2) - 4 * a * c;
    ((delta as f64).sqrt() as i64 - b) / (2 * a)
}

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut ret = HashSet::<[u32; 3]>::new();
    for c in 1..=sum {
        /*
            a + b == sum - c = p
            a ^ 2 + b ^ 2 = c ^ 2 = q
            
              =>
            
            a = p - b
            (p - b) ^ 2 + b ^ 2 = q

              =>

            p ^ 2 - 2pb + 2 * b ^ 2 = q
        */
        let p = (sum - c) as i64; // 7
        let q = c.pow(2) as i64; // 25 2 -14 24
        if p.pow(2) <= q { continue; }
        if solve(2, -2 * p, p.pow(2) - q) <= 0 { continue; }

        let b = solve(2, -2 * p, p.pow(2) - q) as u32;
        let a = sum - c - b;
        if a + b + c == sum && a.pow(2) + b.pow(2) == c.pow(2) && a < b && b < c {
            ret.insert([a, b, c]);
        }
    }
    ret
}
