#[allow(unused_variables)]
fn main() {
    let number = 10;

    if number < 5 {
        println!("condition was true");
        let number = 7;
    } else {
        println!("condition was false");
    }

    println!("num: {}", number);

    println!("0--------------------------------------------------------------------------------");

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };


    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for element in &a {
        println!("the value is: {}", element);
    }
}
