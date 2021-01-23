use rayon::ThreadPoolBuilder;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let pool = ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .unwrap();

    let (tx, rx) = std::sync::mpsc::channel();

    for s in input {
        let str = s.to_string();
        let tx = tx.clone();
        pool.spawn(move || {
            let mut map = HashMap::new();
            for c in str.to_lowercase().chars() {
                if c.is_alphabetic() {
                    *map.entry(c).or_insert(0) += 1;
                }
            }
            tx.send(map).unwrap();
        });
    }
    drop(tx);
    let maps: Vec<HashMap<char, usize>> = rx.into_iter().collect();
    let mut result = HashMap::new();
    for map in maps {
        for c in map.keys() {
            *result.entry(*c).or_insert(0) += map.get(c).unwrap();
        }
    }
    result
}
