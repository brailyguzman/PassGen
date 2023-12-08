use rand::Rng;
use std::io::{self, Write};

fn main() {
    let all_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

    print!("[+] Enter Password Length: ");
    io::stdout().flush().unwrap(); // Flush the output buffer

    let mut length = String::new();
    std::io::stdin()
        .read_line(&mut length)
        .expect("Didn't receive Input");

    let length: u32 = length.trim().parse().expect("Please enter a number");

    let mut password = String::new();

    for _i in 0..length {
        let index = rand::thread_rng().gen_range(0..all_chars.len());
        password.push(all_chars.chars().nth(index).unwrap());
    }

    println!("Password: {}", password);
}
