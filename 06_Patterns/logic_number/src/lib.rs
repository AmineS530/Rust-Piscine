pub fn number_logic(num: u32) -> bool {
    let n_len = num_len(num) as u32;

    split_nums(num).iter().map(|d| d.pow(n_len)).sum::<u32>() == num
}

fn num_len(mut num: u32) -> u8 {
    let mut len: u8 = 1;
    while num > 10 {
        len += 1;
        num /= 10;
    }
    len
}
fn split_nums(mut num: u32) -> Vec<u32> {
    let mut u_vec = Vec::<u32>::new();
    let n_len = num_len(num);
    for _ in 0..n_len {
        u_vec.insert(0, num % 10);
        num /= 10;
    }
    u_vec
}
