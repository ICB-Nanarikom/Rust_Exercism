pub fn square_of_sum(n: u128) -> u128 {
    (n * (n + 1) / 2).pow(2)
}

pub fn sum_of_squares(n: u128) -> u128 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u128) -> u128 {
    square_of_sum(n) - sum_of_squares(n)
}
