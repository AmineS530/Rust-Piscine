pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exf = c as f64;
    (c, exf.exp(), exf.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let str_vec = a
        .split_whitespace()
        .filter_map(|nb| nb.parse::<f64>().ok())
        .map(|nb| nb.exp().to_string())
        .collect::<Vec<String>>()
        .join(" ");
    (a, str_vec)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let nbs_vec = b
        .iter()
        .map(|&num| (num as f64).ln())
        .collect();
    (b, nbs_vec)
}
