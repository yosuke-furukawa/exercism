pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = vec![];
    if len == 0 {
        for _ in digits.as_bytes() {
            result.push("".to_string());
        }
        result.push("".to_string());
        return result;
    }
    for s in digits.as_bytes().windows(len) {
        result.push(std::str::from_utf8(&s).unwrap_or("").to_string());
    }
    result
}
