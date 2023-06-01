use std::collections::HashSet;

fn is_match(a: char, b: char) -> bool {
    match (a, b) {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        _ => false,
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::<char>::new();
    let brackets = HashSet::from(['(', ')', '[', ']', '{', '}']);
    for c in string.chars().filter(|c| brackets.contains(c) ) {
        if stack.is_empty() == false && is_match(stack.last().unwrap().to_owned(), c) {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    stack.is_empty()
}
