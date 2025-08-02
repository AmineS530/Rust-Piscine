pub fn stars(n: u32) -> String {
    let n = n.pow(2);
    let mut s = String::new();
    for _ in 0..n {
        s.push('*');
    }
    s
}
