pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result = Vec::new();
    for name in names.iter() {
        let mut initial = String::new();
        for word in name.split_whitespace() {
            if let Some(first_char) = word.chars().next() {
                initial.push(first_char.to_ascii_uppercase());
                initial.push('.');
                initial.push(' ');
            }
        }
        initial.pop();
        result.push(initial);
    }
    result
}
