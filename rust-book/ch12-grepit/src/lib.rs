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
    pub fn from_args(args: env::Args) -> Result<Config, &'static str> {
        // first argument is the program name. let's skip it.
        let mut args = args.skip(1);

        let query = args.next().ok_or("Didn't provide a query string")?;
        let file_name = args.next().ok_or("Didn't provide a file name")?;

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { file_name, query, case_sensitive })
    }
}

fn search<'a>(query: &str, content: &'a str, contains: fn(&str, &str) -> bool) -> Vec<&'a str> {
    content.lines()
        .filter(|&line| contains(line, query))
        .collect()
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
