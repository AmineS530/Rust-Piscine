pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    text.split_whitespace()
        .map(|word| {
            let chars: Vec<char> = word.chars().collect();

            // Case 1: Starts with a vowel
            if vowels.contains(&chars[0]) {
                format!("{word}ay")
            }
            // Case 2: Starts with consonant + "qu" (like "squash")
            else if chars.len() > 2 && !vowels.contains(&chars[0]) && chars[1] == 'q' && chars[2] == 'u' {
                let rest: String = chars[3..].iter().collect();
                let prefix: String = chars[..3].iter().collect(); // consonant + "qu"
                format!("{rest}{prefix}ay")
            }
            // Case 3: Starts with "qu" (like "quick")
            else if chars.len() > 1 && chars[0] == 'q' && chars[1] == 'u' {
                let rest: String = chars[2..].iter().collect();
                format!("{rest}quay")
            }
            // Case 4: Starts with consonants until first vowel
            else {
                let pos = chars.iter()
                               .position(|c| vowels.contains(c))
                               .unwrap_or(0); // if no vowel found

                let (head, tail) = chars.split_at(pos);
                let new_word: String = tail.iter().chain(head.iter()).collect();
                format!("{new_word}ay")
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
