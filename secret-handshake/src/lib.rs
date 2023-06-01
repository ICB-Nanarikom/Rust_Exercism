fn bit(n: u8, i: u8) -> u8 {
    (n >> i) & 1
}

pub fn actions(n: u8) -> Vec<&'static str> {
    let mut ret = Vec::<&str>::new();
    for i in 0..4 {
        if bit(n, i) == 1 {
            ret.push(match i {
                0 => "wink",
                1 => "double blink",
                2 => "close your eyes",
                _ /* 3 */ => "jump",
            });
        }
    }
    if bit(n, 4) == 1 {
        ret = ret.into_iter().rev().collect::<Vec<&str>>();
    }
    ret
}
