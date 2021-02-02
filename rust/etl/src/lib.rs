use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut map = BTreeMap::new();
    for v in h.keys() {
        let keys = h.get(v).unwrap();
        for k in keys {
            let mut newk = *k;
            newk.make_ascii_lowercase();
            map.insert(newk, *v);
        }
    }
    map
}
