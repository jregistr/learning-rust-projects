use std::{fs, env};
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_str = fs::read_to_string(&config.file_name)?;
    let query = &config.query;

    let c = if config.case_sensitive { line_contains } else { line_contains_case_insensitive };

    for line in search(query, &file_str, c) {
        println!("{}", line);
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    file_name: String,
    query: String,
    case_sensitive: bool,
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough args");
        }
        let (query, fnn) = (&args[1], &args[2]);
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { file_name: fnn.clone(), query: query.clone(), case_sensitive })
    }
}

fn search<'a>(query: &str, content: &'a str, contains: fn(&str, &str) -> bool) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in content.lines() {
        if contains(line, query) {
            res.push(line);
        }
    }
    res
}

fn line_contains(line: &str, query: &str) -> bool {
    line.contains(query)
}

fn line_contains_case_insensitive(line: &str, query: &str) -> bool {
    line.to_lowercase().contains(&query.to_lowercase())
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
        assert_eq!(vec!["pick all three."], search(query, content, line_contains));
    }

    #[test]
    fn search_case_insensitively() {
        let query = "ThReE";
        let content = "\
Rust: Safety, Speed, Productivity.
pick all three.
        ";
        assert_eq!(vec!["pick all three."], search(query, content, line_contains_case_insensitive));
    }
}
