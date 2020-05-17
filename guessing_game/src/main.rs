use rand::Rng;
use std::cmp::Ordering;

fn main() {


    let min = 1;
    let max = rand::thread_rng().gen_range(10, 101);

    let secret_num = rand::thread_rng()
        .gen_range(min, max);

    println!("Can you guess the number!!? It's between {min} and {max}", min=min, max=max);

    use std::io;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {num}", num = guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Yup, that was correct, you win!!");
                break;
            }
        }
    }
}
