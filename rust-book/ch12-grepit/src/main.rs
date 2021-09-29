use std::{process, env};

use ch12_grepit::{Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(&args)
        .unwrap_or_else(|err| {
            println!("Problem processing args: {}", err);
            process::exit(1)
        });
    if let Err(e) = ch12_grepit::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
