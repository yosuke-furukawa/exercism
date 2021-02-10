use std::collections::HashSet;

pub fn find<T, S>(array: S, key: T) -> Option<usize>
where
    T: std::cmp::Ord,
    S: AsRef<[T]>,
{
    let array = array.as_ref();
    let mut left = 0;
    let mut right = array.len();
    let mut visited = HashSet::new();
    while left < right {
        let mid = (left + right) / 2;
        if visited.contains(&mid) {
            break;
        }
        visited.insert(mid);
        match &array[mid] {
            m if m.gt(&key) => {
                right = mid;
            }
            m if m.lt(&key) => {
                left = mid;
            }
            _ => {
                return Some(mid);
            }
        }
    }
    None
}
