use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut map = HashMap::new();
    match nucleotide {
        'A' | 'G' | 'C' | 'T' => (),
        _ => return Err(nucleotide),
    }
    for c in dna.chars() {
        match c {
            'A' | 'G' | 'C' | 'T' => *map.entry(c).or_insert(0_usize) += 1,
            _ => return Err(c),
        }
    }
    map.get(&nucleotide).or(Some(&0)).cloned().ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = vec![('A', 0), ('C', 0), ('G', 0), ('T', 0)]
        .into_iter()
        .collect();
    for c in dna.chars() {
        match c {
            'A' | 'G' | 'C' | 'T' => *map.entry(c).or_insert(0_usize) += 1,
            _ => return Err(c),
        }
    }
    Ok(map)
}
