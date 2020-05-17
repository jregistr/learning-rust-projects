#[allow(unused_variables)]
fn main() {
    println!("Hello, world!");

    another_function(343221);

    even_another_function(32, 551);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn even_another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn foobar() -> i32 {
    1000
}
