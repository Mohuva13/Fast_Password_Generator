#![allow(non_snake_case)]

use generator::password_generator::gen_by_len as gen_by_len;
use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;



mod generator;

fn main() {
    // input password length
    let mut password_length = String::new();
    println!("Input password length: ");
    std::io::stdin().read_line(&mut password_length).expect("Failed to read line");
    let password_length: i32 = password_length.trim().parse().expect("Please type a number!");


    // Password_string
    // Generate password
    let password_string = String::from(gen_by_len(password_length));



    // Copy password to clipboard by copypasta crate
    let mut ctx = ClipboardContext::new().unwrap();
    let _ = ctx.get_contents();
    ctx.set_contents(password_string.to_string()).unwrap();
    let _ = ctx.get_contents();


    // Print password
    println!("\n");
    println!("Generated password:");
    println!("{}", password_string);
    println!("\n Password has been copied to clipboard!");
}
