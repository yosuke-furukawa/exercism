pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut mark = vec![true; (upper_bound as usize) + 1];
    for start in 2..upper_bound as usize {
        if mark[start] == true {
            for i in (start * 2..mark.len()).step_by(start) {
                mark[i] = false;
            }
        }
    }
    mark.iter()
        .enumerate()
        .skip(2)
        .filter(|(_, value)| **value == true)
        .map(|(idx, _)| idx as u64)
        .collect()
}
