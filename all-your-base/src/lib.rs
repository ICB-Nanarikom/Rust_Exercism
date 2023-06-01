#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

fn k_to_dec(n: &[u32], k: u32,) -> Result<u32, Error> {
    if k <= 1 {
        return Result::Err(Error::InvalidInputBase);
    }

    let mut sum = 0;
    for i in n.iter() {
        sum = sum * k + i;
        if *i >= k {
            return Result::Err(Error::InvalidDigit(*i));
        }
    }
    Result::Ok(sum)
}

fn dec_to_k(mut sum: u32, k: u32) -> Result<Vec<u32>, Error> {
    if k <= 1 {
        return Result::Err(Error::InvalidOutputBase);
    }
    if sum == 0 {
        return Result::Ok(vec!(0));
    }

    let mut v = Vec::<u32>::new();
    while sum > 0 {
        v.push(sum % k);
        sum /= k;
    }
    Result::Ok(v.into_iter().rev().collect::<Vec<_>>())
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    match k_to_dec(number, from_base) {
        Ok(sum) => dec_to_k(sum, to_base),
        Err(e) => Result::Err(e),
    }
}
