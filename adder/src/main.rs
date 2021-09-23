#![allow(unused)]

use add_one::{add_one, add_rand};

fn main() {
    let num = 10;
    println!("Hello world and {} Plus one is {}", num, add_one(num));
    println!("or we make random {}", add_rand(num));
}
