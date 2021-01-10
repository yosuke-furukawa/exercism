use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    let lowword = word.to_string().to_lowercase();
    let mut target: Vec<char> = lowword.chars().collect();
    target.sort_unstable_by(|a, b| a.to_string().cmp(&b.to_string()));

    for anagram in possible_anagrams {
        let lowanagram = anagram.to_string().to_lowercase();
        let mut chars: Vec<char> = lowanagram.chars().collect();
        chars.sort_unstable_by(|a, b| a.to_string().cmp(&b.to_string()));
        
        if lowword != lowanagram && target == chars {
            result.insert(*anagram);
        }
    }
    result
}
