#[allow(unused_variables)]
fn main() {
    /*
    let x = 5;
    println!("The value of x is: {}", x);
    // Won't compile since x is immutable
    x = 6;
    println!("The value of x is: {}", x);
     */
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const FOO: i32 = 1200;
    println!("The value of FOO is: {}", FOO);

    //shadowed -
    let x = 100;
    println!("The value of x is: {}", x);

    let mut spaces = "   ";
    spaces = "      ";
    let spaces = spaces.len();

    println!("Spaces size: {}", spaces);


    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    let quot2 = 56 / 10;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let str_test = "this is a str";


    //declaring an array
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // declare an array of 5 items filled with 0s;
    let zeroes = [0; 5];
    let f = zeroes[3];

    // inferred
    let zeroes = [0, 0, 0, 0, 0];

    // foobar(10); //panic

    control_flow();
    for_loops();
}

#[allow(unused_variables)]
fn foobar(ind: usize) {
    let zeroes = [0, 0, 0, 0, 0];
    let f = zeroes[ind];
}

// returns last expression. if a semicolon was added, it'd be a statement instead
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn control_flow() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if is an expression
    let n2 = if number < 5 { 10 } else { 80 };

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number & 1 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}


fn loops() {
    let mut count = 1;
    // This is like a while(true) in java. Runs until an explicit break
    loop {
        count += 1;
        if count > 40 { break; }
    }

    //Returning values from loops

    let mut count = 1;
    // add the value we wish to return after break;
    let upped = loop {
        count *= 4;

        if count > 100 {
            break count * 2;
        }
    };
    println!("The result is {}", upped);
}

fn while_loop() {
    let mut count_down = 3;

    while count_down != 0 {
        println!("{}!", count_down);

        count_down -= 1;
    }
    println!("Houston, we have lift-off");
}

fn for_loops() {
    // use these with an iterator
    //ex: an array iterator
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // loop over range. similar to for(int i = 0; i ....) in java
    for index in (0..4) {
        println!("{}", index);
    }

    // this range does not include the value 4. Will have 1, 2, 3
    let non_inclusive_range = 1..4;
    let inclusive_range = 1..= 4;
    println!("{:?}", non_inclusive_range.it);
    println!("{:?}", inclusive_range);
}