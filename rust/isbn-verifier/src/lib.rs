/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut digit_count = 10;
    let mut answer = 0;
    for s in isbn.chars() {
        match s {
            '-' => continue,
            _ if s.is_numeric() => {
                if digit_count <= 0 {
                    return false;
                }
                let n = s as i32 - 48;
                answer += n * digit_count;
                digit_count -= 1;
            }
            'X' => {
                if digit_count != 1 {
                    return false;
                }
                answer += 10 * digit_count;
                digit_count -= 1;
            }
            _ => return false,
        }
    }
    digit_count == 0 && answer % 11 == 0
}
