#[derive(PartialEq)]
enum SPCHAR {
    BRACKET,
    BRACE,
    PAREN,
    NONE,
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut start = 0;
    let mut end = string.len() - 1;
    let chars = string.chars();
    let mut current = SPCHAR::NONE;
    let mut result = false;
    while start < end {
        if current == SPCHAR::NONE {
            current = match chars[start] {
                Some("[") => SPCHAR::BRACKET,
                Some("{") => SPCHAR::BRACE,
                Some("(") => SPCHAR::PAREN,
                _ => SPCHAR::NONE,
            }
        }
    }
    result
}
