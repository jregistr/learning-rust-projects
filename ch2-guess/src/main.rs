#![allow(unused)]

use std::io;
use rand;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number. It's somewhere between 1 and 100");

    let secret_number = rand::thread_rng().gen_range(0..101);
    println!("Please type a number!");
    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input = user_input.trim();
        if user_input == "q" {
            println!("Ok, we're done playing then");
            break;
        }

        let guess: i32 = match user_input.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Bad input: {}, it should be a number", user_input);
                continue;
            }
        };
        println!("You guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Congrats! You've won!!!");
                break;
            }
        }
    }
}
