pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut result = Vec::new();
    let mut divided = 2;
    while num > 1 {
        if num % divided == 0 {
            result.push(divided);
            num = num / divided;
        } else {
            divided += 1;
        }
    }
    result
}
