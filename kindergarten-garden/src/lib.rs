pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    let mut it = _diagram.split('\n');
    let v1 = it.next().unwrap();
    let v2 = it.next().unwrap();
    let id = (_student.chars().nth(0).unwrap() as u8) - ('A' as u8);
    
    vec![
        v1.chars().nth(2 * id as usize).unwrap(),
        v1.chars().nth(2 * id as usize + 1).unwrap(),
        v2.chars().nth(2 * id as usize).unwrap(),
        v2.chars().nth(2 * id as usize + 1).unwrap(),
    ].iter().map(|c| match c {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => "",
    }).collect::<Vec<&'static str>>()
}
