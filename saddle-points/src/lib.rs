fn r_max(input: &[Vec<u64>], r: usize, m: usize) -> u64 {
    (0..m).fold(u64::MIN, |mx, i| std::cmp::max(mx, input[r][i]))
}
fn c_min(input: &[Vec<u64>], c: usize, n: usize) -> u64 {
    (0..n).fold(u64::MAX, |mn, i| std::cmp::min(mn, input[i][c]))
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    let n = input.len();
    if n == 0 { return ret; }
    let m = input[0].len();
    if m == 0 { return ret; }

    for i in 0..n {
        for j in 0..m {
            if r_max(input, i, m) == c_min(input, j, n) {
                ret.push((i, j));
            }
        }
    }
    ret
}
