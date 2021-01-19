fn digit(n: u32) -> u32 {
    if n >= 9 {
        return n - 9;
    }
    n
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut result = 0;
    let code = code.replace(" ", "");
    if code.len() <= 1 {
        return false;
    }
    for (i, v) in code.chars().rev().enumerate() {
        result += match (i, v) {
            (_, v) if !v.is_numeric() => return false,
            (i, v) if i % 2 == 1 => digit(v.to_digit(10).unwrap() * 2),
            _ => v.to_digit(10).unwrap(),
        };
    }
    result % 10 == 0
}
