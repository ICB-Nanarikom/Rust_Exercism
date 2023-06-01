pub fn number(user_number: &str) -> Option<String> {
    let user_number = match user_number.strip_prefix("1") {
        Some(v) => v,
        None => user_number,
    };
    let user_number = match user_number.strip_prefix("+1") {
        Some(v) => v,
        None => user_number,
    };
    if user_number.chars().any(|c| c.is_numeric() == false && c != '(' && c != ')' && c != '-' && c != '.' && c != ' ') {
        return None;
    }

    let user_number = user_number.chars().filter(|c| c.is_numeric()).collect::<String>();
    if user_number.len() != 10 {
        return None;
    }

    let c = user_number.chars().nth(0).unwrap();
    if c == '0' || c == '1' {
        return None;
    }
    let c = user_number.chars().nth(3).unwrap();
    if c == '0' || c == '1' {
        return None;
    }
    
    Some(user_number)
}
