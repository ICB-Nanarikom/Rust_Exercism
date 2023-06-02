use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut ret = BTreeMap::<char, i32>::new();
    for (i, clist) in h {
        for c in clist {
            ret.insert(((*c as u8) + 32u8) as char, *i);
        }
    }
    ret
}
