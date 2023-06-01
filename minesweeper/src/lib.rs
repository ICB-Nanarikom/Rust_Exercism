use std::cmp::max;
use std::cmp::min;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut ret = Vec::<String>::new();
    for (x, row) in minefield.iter().enumerate() {
        let mut ret_row = String::new();
        for (y, c) in (*row).chars().enumerate() {
            if c == '*' {
                ret_row += "*";
            } else {
                let mut cnt = 0;
                let xl = max(0, x as i32 - 1) as usize;
                let xr = min(minefield.len() as i32 - 1, x as i32 + 1) as usize;
                let yl = max(0, y as i32 - 1) as usize;
                let yr = min(row.len() as i32 - 1, y as i32 + 1) as usize;
                for xt in &minefield[xl..=xr] {
                    for yt in xt[yl..=yr].chars() {
                        cnt += if yt == '*' {1} else {0};
                    }
                }
                if cnt > 0 {
                    ret_row += cnt.to_string().as_str()
                } else {
                    ret_row += " ";
                }
            }
        }
        ret.push(ret_row);
    }
    ret
}
