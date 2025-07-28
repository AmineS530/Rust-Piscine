pub fn delete_and_backspace(s: &mut String) {
    let mut res = String::new();
    for char in s.chars() {
        if char == '+' {
            char.next();
            continue;
        } else if char == '-' {
            res.pop();
            continue;
        }
        res.push(char);
    }
    *s = res;
}

// pub fn do_operations(v: &mut [String]) {}
// pub fn delete_and_backspace(s: &mut String) {
//     let mut res = String::new();
//     let mut chars = s.chars().peekable();

//     while let Some(c) = chars.next() {
//         match c {
//             '-' => {
//                 res.pop(); // remove last added char
//             }
//             '+' => {
//                 chars.next(); // skip the next character
//             }
//             _ => {
//                 res.push(c);
//             }
//         }
//     }

//     *s = res;
// }
