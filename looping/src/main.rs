fn main() {
    let mut counter = 1;
    let riddle =
        "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        println!("compare : {}", input.trim() == "The letter e");
        if input.trim() == "The letter e" {
            break;
        }
        counter += 1;
    }
    println!("Number of trials: {}", counter);
}
