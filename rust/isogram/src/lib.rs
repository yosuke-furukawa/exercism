use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    for c in candidate.chars() {
        let c = c.to_ascii_lowercase();
        match c {
            ' ' | '-' => continue,
            _ if set.contains(&c) => return false,
            _ => {
                set.insert(c);
            }
        }
    }
    true
}
