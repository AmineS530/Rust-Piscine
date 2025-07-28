pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

pub fn change_case(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars() {
        if c.is_uppercase() {
            for up_c in c.to_lowercase() {
                result.push(up_c);
            }
        } else if c.is_lowercase() {
            for low_c in c.to_uppercase() {
                result.push(low_c);
            }
        } else {
            result.push(c);
        }
    }

    result
}
