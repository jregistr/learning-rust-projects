use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = (&args[0], &args[1]);

    let file_str = fs::read_to_string(&filename)
        .expect(format!("No file found at {}", &filename).as_str());

    println!("{}", file_str);
}
