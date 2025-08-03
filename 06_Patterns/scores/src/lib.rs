pub fn score(s: &str) -> u64 {
let mut value: u64 = 0;
    for c in s.chars() {
        match c.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => value += 1,
            'D' | 'G' => value += 2,
            'B' | 'C' | 'M' | 'P' => value += 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => value += 4,
            'K' => value += 5,
            'J' | 'X' => value += 8,
            'Q' | 'Z' => value += 10,
            _ => (),
        }
    }
    value
}
