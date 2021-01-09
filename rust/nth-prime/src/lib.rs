pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2, 3, 5, 7, 11, 13];
    let mut current = 15;
    let mut length = primes.len() as u32;
    while length <= n {
        let mut found = false;
        for v in &primes {
            if current % v == 0 {
                found = true;
                break;
            }
        }

        if found {
            current = current + 2;
            continue;
        }
        primes.push(current);
        length = length + 1;
    }
    
    return *primes.get(n as usize).unwrap_or(&0);
}
