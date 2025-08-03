pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return "Just say something!";
    }

    let is_question = trimmed.ends_with('?');
    let mut all_upper = true;

    for c in trimmed.chars() {
        if c.is_alphabetic() {
            if !c.is_uppercase() {
                all_upper = false;
            }
        }
    }

    if all_upper && is_question {
        "Quiet, I am thinking!"
    } else if all_upper {
        "There is no need to yell, calm down!"
    } else if is_question {
        "Sure."
    } else {
        "Interesting"
    }
}
