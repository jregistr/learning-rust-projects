use std::{env, fs, process};
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(&args)
        .unwrap_or_else(|err| {
            println!("Problem processing args: {}", err);
            process::exit(1)
        });
    run(config);
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let file_str = fs::read_to_string(&file_name)?;

    println!("{}", file_str);
    Ok(())
}

#[derive(Debug)]
struct Config {
    file_name: String,
    query: String,
}

impl Config {
    fn from_args(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough args");
        }
        let (query, fnn) = (&args[1], &args[2]);
        Ok(Config { file_name: fnn.clone(), query: query.clone() })
    }
}