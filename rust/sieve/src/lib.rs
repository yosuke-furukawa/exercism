pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut nums = (2..=upper_bound).collect::<Vec<u64>>();
    let mut primes = vec![];
    while !nums.is_empty() {
        let prime = nums[0];
        nums = nums.iter().filter(|n| *n % prime != 0).cloned().collect();
        primes.push(prime);
    }
    primes
}
