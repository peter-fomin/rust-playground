use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let sorted_word = sorted_chars(&word);
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    for anagram in possible_anagrams {
        let anagram_lower = anagram.to_lowercase();
        if anagram_lower != word && sorted_chars(&anagram_lower) == sorted_word {
            anagrams.insert(anagram);
        }
    }
    anagrams
}

pub fn sorted_chars(word: &str) -> Vec<char> {
    let mut result: Vec<char> = word.chars().collect();
    result.sort();
    result
}
