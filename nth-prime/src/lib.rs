pub fn nth(n: u32) -> u32 {
    const N: usize = 1000005;
    let mut flag = Vec::<u32>::new();
    let mut pri = Vec::<u32>::new();
    flag.resize(N, 0);
    pri.resize(N, 0);
    let mut cnt = 0;
    for i in 2..N{
        if flag[i] == 0 {
            pri[cnt] = i as u32;
            cnt += 1;
        }
        let mut j = 0;
        while ((i * pri[j] as usize) as u32) < (N as u32) {
            flag[i * pri[j] as usize] = 1;
            if i % pri[j] as usize == 0 { break; }
            j += 1;
        }
    }
    pri[n as usize]
}
