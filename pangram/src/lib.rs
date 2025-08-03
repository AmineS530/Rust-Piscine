use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut letters: HashSet<char> = HashSet::new();

    for c in s.chars() {
        if c.is_alphabetic() {
            let lower = c.to_lowercase().next().unwrap();
            letters.insert(lower);
        }
    }

    letters.len() == 26
}
