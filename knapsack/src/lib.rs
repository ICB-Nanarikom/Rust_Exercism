pub struct Item {
    pub weight: u32,
    pub value: u32,
}

use std::cmp::max;
pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut dp = Vec::<u32>::new();
    dp.resize((max_weight + 1) as usize, 0);
    for item in items {
        for j in (item.weight..=max_weight).rev() {
            dp[j as usize] = max(dp[j as usize], dp[(j - item.weight) as usize] + item.value);
        }
    }
    dp.into_iter().fold(0, |mx, i| max(mx, i))
}
