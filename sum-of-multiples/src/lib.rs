pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut ret = 0;
    for i in 1..limit {
        let mut succ = false;
        for factor in factors {
            if (*factor > 0) && (i % factor == 0) {
                succ = true;
            }
        }
        ret += if succ { i } else { 0 }; 
    }
    ret
}
