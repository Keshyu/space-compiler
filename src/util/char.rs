pub fn is_alnum(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

pub fn is_alpha(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' => true,
        _ => false,
    }
}

pub fn is_digit(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}
