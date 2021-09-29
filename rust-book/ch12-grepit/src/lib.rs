use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_str = fs::read_to_string(&config.file_name)?;
    let query = &config.query;

    for line in search(query, &file_str) {
        println!("{}", line);
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    file_name: String,
    query: String,
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough args");
        }
        let (query, fnn) = (&args[1], &args[2]);
        Ok(Config { file_name: fnn.clone(), query: query.clone() })
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_find() {
        let query = "three";
        let content = "\
Rust: Safety, Speed, Productivity.
pick all three.
        ";
        assert_eq!(vec!["pick all three."], search(query, content));
    }
}
