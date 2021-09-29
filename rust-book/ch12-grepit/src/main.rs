use std::{process, env};

use ch12_grepit::{Config};

fn main() {
    let config = Config::from_args(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem processing args: {}", err);
            process::exit(1)
        });
    if let Err(e) = ch12_grepit::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
