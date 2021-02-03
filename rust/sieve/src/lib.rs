pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut nums: Vec<u64> = (2..=upper_bound).collect();
    let mut primes = vec![];
    while !nums.is_empty() {
        let prime = nums[0];
        nums.retain(|&n| n % prime != 0);
        primes.push(prime);
    }
    primes
}
