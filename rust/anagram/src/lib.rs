use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    let lowword = word.to_string().to_lowercase();
    let mut target: Vec<char> = lowword.chars().collect();
    target.sort_unstable();

    for anagram in possible_anagrams {
        if anagram.len() != word.len() {
            continue;
        }

        let lowanagram = anagram.to_lowercase();

        if lowanagram == lowword {
            continue;
        }

        let mut chars: Vec<char> = lowanagram.chars().collect();
        chars.sort_unstable();
        
        if target == chars {
            result.insert(*anagram);
        }
    }
    result
}
