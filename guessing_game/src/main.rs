fn random_int() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    seed ^= seed << 21;
    seed ^= seed >> 35;
    seed ^= seed << 4;
    seed
}

fn read_int() -> u8 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let answer = ((random_int()) % 100) as u8;
    loop {
        let current_int = read_int();
        if answer == current_int {
            break;
        }
        println!("{}", if answer > current_int { ">" } else { "<" });
    }
    println!("Congratulations! The hidden number is {}", answer)
}