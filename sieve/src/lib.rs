pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut flag = Vec::<u64>::new();
    let mut pri = Vec::<u64>::new();
    flag.resize(upper_bound as usize + 1, 0);

    for i in 2..=upper_bound as usize {
        if flag[i] == 0 {
            // pri[cnt] = i as u32;
            pri.push(i as u64);
        }
        let mut j = 0;
        while (i as u64) * pri[j] <= upper_bound {
            flag[i * pri[j] as usize] = 1;
            if (i as u64) % pri[j] == 0 { break; }
            j += 1;
        }
    }
    pri
}
