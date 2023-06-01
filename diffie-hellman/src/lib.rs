fn pow(mut n: u64, mut m: u64, p: u64) -> u64 {
    let mut ret: u64 = 1;
    while m > 0 {
        if m & 1 > 0 {
            ret = ret * n % p;
        }
        n = n * n % p;
        m >>= 1;
    }
    ret
}

pub fn private_key(_p: u64) -> u64 {
    2
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    pow(b_pub, a, p)
}
