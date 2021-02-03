pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut nums = (2..=upper_bound).collect::<Vec<u64>>();
    let mut prime = 2;
    let mut primes = vec![prime];
    while prime < upper_bound {
        nums = nums.iter().filter(|n| { *n % prime != 0 }).cloned().collect();
        if nums.len() == 0 {
            break;
        }
        prime = nums[0];
        primes.push(prime);
    }
    primes
}
