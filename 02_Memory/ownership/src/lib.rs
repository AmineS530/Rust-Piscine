fn find(v: &str, pat: char) -> usize {
    match v.find(pat) {
        Some(idx) => idx,
        None => usize::MAX,
    }
}

pub fn first_subword(mut s: String) -> String {
    if s.contains('_') {
        let idx = find(&s, '_');
        return s.drain(..idx).collect();
    } else {
        for (i, c) in s.chars().enumerate().skip(1) {
            if c.is_uppercase() {
                return s.drain(..i).collect();
            }
        }
    }
    s
}
