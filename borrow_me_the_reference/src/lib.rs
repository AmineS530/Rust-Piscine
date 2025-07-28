pub fn delete_and_backspace(s: &mut String) {
    let mut res = String::new();
    let mut chars = s.chars();
    let mut delete_counter = 0;

    while let Some(c) = chars.next() {
        match c {
            '+' => {
                delete_counter += 1;
            }
            '-' => {
                res.pop();
            }
            _ => {
                if delete_counter > 0 {
                    delete_counter -= 1;
                } else {
                    res.push(c); // keep the character
                }
            }
        }
    }

    *s = res;
}

pub fn do_operations(v: &mut [String]) {
    for op in v.iter_mut() {
        let parts: Vec<&str> = if op.contains('+') {
            op.split('+').collect()
        } else {
            op.split('-').collect()
        };
        let first = parts[0].parse::<i32>().unwrap_or(0);
        let second = parts[1].parse::<i32>().unwrap_or(0);

        let result = if op.contains('+') { first + second } else { first - second };
        *op = result.to_string();
    }
}
