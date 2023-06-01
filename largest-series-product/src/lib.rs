#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    match string_digits.chars().find(|c| c.is_numeric() == false) {
        Some(c) => { return Err(Error::InvalidDigit(c)); },
        None => {},
    }

    Ok(string_digits.as_bytes().windows(span)
        .map(|s| s.to_vec().into_iter().map(|i| (i - '0' as u8) as u64).product())
        .max().unwrap())
}
