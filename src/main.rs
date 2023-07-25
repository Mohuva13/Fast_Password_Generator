#![allow(non_snake_case)]

use generator::password_generator::gen_by_len as gen_by_len;

mod generator;

fn main() {
    // input password length
    let mut password_length = String::new();
    println!("Input password length: ");
    std::io::stdin().read_line(&mut password_length).expect("Failed to read line");
    let password_length: i32 = password_length.trim().parse().expect("Please type a number!");

    // generate password
    println!("\n");
    println!("Generated password:");
    println!("{}", gen_by_len(password_length));
}
