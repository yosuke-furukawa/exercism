pub fn is_armstrong_number(num: u32) -> bool {
    let mut n = num;
    let mut res = Vec::new();
    let mut count = 0;
    while n > 0 {
        let r = n % 10;
        res.push(r);
        n = n / 10;
        count += 1;
    }

    let result = num == res.iter().fold(0, |a, b| a + b.pow(count));
    result
}
