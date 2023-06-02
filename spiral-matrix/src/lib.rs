fn check(ret: &Vec<Vec<u32>>, size: i32, x: i32, y: i32) -> bool {
    0 <= x && x <= size - 1 && 0 <= y && y <= size - 1 && ret[x as usize][y as usize] == 0
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return Vec::new();
    }

    let mut ret = vec![vec![0; size as usize]; size as usize];
    let mut it: std::iter::Cycle<std::vec::IntoIter<(i32, i32)>> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter().cycle();
    let (mut x, mut y) = (0, 0);
    let mut c = 1;
    ret[x][y] = c;

    loop {
        let (dx, dy) = it.next().unwrap();
        while check(&ret, size as i32, (x as i32) + dx, (y as i32) + dy) {
            x = (x as i32 + dx) as usize;
            y = (y as i32 + dy) as usize;
            c += 1;
            ret[x][y] = c;
        }
        if c == size.pow(2) { break; }
    }

    ret
}
