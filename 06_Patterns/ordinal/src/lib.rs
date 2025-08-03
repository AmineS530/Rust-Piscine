pub fn num_to_ordinal(x: u32) -> String {
    if (11..=13).contains(&(x % 100)) {
        format!("{}th", x)
    } else {
        match x % 10 {
            1 => format!("{}st", x),
            2 => format!("{}nd", x),
            3 => format!("{}rd", x),
            _ => format!("{}th", x),
        }
    }
}
