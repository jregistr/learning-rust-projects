#![allow(unused)]

use std::{io, fs::File, io::Read};

fn main() {
    let hf = File::open("ch9-errors/hello.txt");
    let mut hf = hf.unwrap();

    let mut buf = String::new();
    hf.read_to_string(&mut buf).unwrap();
    let buf = buf;
    println!("{}", buf);

    let mut buf = String::new();
    let file = File::open("ch9-errors/hello.txt");

    use std::io::ErrorKind;
    let mut file = match file {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("ch9-errors/hello.txt") {
                Ok(created) => created,
                Err(_) => panic!("Unable to create file after trying to read it.")
            }
            _ => panic!("Unexpected error reading file.")
        }
    };

    file.read_to_string(&mut buf);
    println!("More wordy error handling: {}", buf);

    let file = File::open("ch9-errors/hello.txt").expect("Something crazy happened!");
    drop(file);
    // T
}

// read file with basic error propagation
fn read_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_file_short() -> Result<String, io::Error> {
    /*let mut f = File::open("hello.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)*/
    let mut buf = String::new();
    File::open("hello.txt")?.read_to_string(&mut buf)?;
    Ok(buf)
    // We can in this case just use fs::readToString
}


