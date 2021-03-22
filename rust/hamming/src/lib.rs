/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    let mut diff = (s1.len() as i32 - s2.len() as i32).abs();
    if diff != 0 {
        return None;
    }
    let len = if s1.len() > s2.len() { s2.len() } else { s1.len() };
    for i in 0..len {
        if c1[i] != c2[i] {
            diff += 1;
        }
    }
    Some(diff as usize)
}
