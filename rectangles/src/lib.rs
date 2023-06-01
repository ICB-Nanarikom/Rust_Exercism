fn c(lines: &[&str], x: usize, y: usize) -> char {
    lines[x].chars().nth(y).unwrap()
}

fn check(lines: &[&str], xl: usize, xr: usize, yl: usize, yr: usize) -> bool {
    if [
        c(lines, xl, yl),
        c(lines, xl, yr),
        c(lines, xr, yl),
        c(lines, xr, yr),
    ].iter().any(|c| "+".contains(*c) == false) {
        return false;
    }
    
    let mut ver = Vec::<char>::new();
    for x in xl..=xr {
        ver.push(c(lines, x, yl));
        ver.push(c(lines, x, yr));
    }
    if ver.iter().any(|c| "+|".contains(*c) == false) {
        return false;
    }

    let mut hor = Vec::<char>::new();
    for y in yl..=yr {
        hor.push(c(lines, xl, y));
        hor.push(c(lines, xr, y));
    }
    if hor.iter().any(|c| "+-".contains(*c) == false) {
        return false;
    }

    true
}

pub fn count(lines: &[&str]) -> u32 {
    let n = lines.len();
    if n == 0 {
        return 0;
    }
    let m = lines[0].len();
    if m == 0 {
        return 0;
    }

    let mut ret = 0;
    for xl in 0..=n - 1 {
        for xr in xl + 1..=n - 1 {
            for yl in 0..=m - 1 {
                for yr in yl + 1..=m - 1 {
                    ret += if check(lines, xl, xr, yl, yr) { 1 } else { 0 };
                }
            }
        }
    }
    ret
}
