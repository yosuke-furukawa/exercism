use std::iter::FromIterator;

pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|f| String::from_iter(f.iter()))
        .collect::<Vec<String>>()
}
