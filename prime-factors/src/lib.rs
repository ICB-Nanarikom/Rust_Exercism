pub fn factors(mut n: u64) -> Vec<u64> {
    let mut ret = Vec::<u64>::new();
    for i in 2..=((n as f64).sqrt() as u64) {
        while n % i == 0 {
            ret.push(i);
            n /= i;
        }
    }
    if n > 1 {
        ret.push(n);
    }
    ret
}
