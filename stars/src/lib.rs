pub fn stars(n: u32) -> String {
    let n = 2_u32.pow(n);
    let mut s = String::new();
    for _ in 0..n {
        s.push('*');
    }
    s
}
