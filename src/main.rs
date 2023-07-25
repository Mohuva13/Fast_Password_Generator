#![allow(non_snake_case)]

use generator::password_generator::gen_by_len as gen_by_len;

mod generator;

fn main() {
    println!("{}", gen_by_len(10));
}
