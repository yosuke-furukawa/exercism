pub fn collatz(mut n: u64) -> Option<u64> {
    if n < 1 {
        return None;
    }
    let mut result = 0;
    while n != 1 {
        match n {
            _ if n % 2 == 0 => n /= 2,
            _ => n = n * 3 + 1,
        }
        result += 1;
    }
    Some(result)
}
