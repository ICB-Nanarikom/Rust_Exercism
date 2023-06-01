pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    if key < array.first().unwrap().to_owned() || array.last().unwrap().to_owned() < key {
        return None;
    }

    let mut l = 0 as i32;
    let mut r = (array.len() - 1) as i32;
    while l <= r {
        let mid = ((l + r + 1) / 2) as usize;
        if array[mid] < key {
            l = mid as i32 + 1;
        } else {
            r = mid as i32 - 1;
        }
    }
    if array[l as usize] == key { Some(l as usize) } else { None }
}
