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

    const FOO:i32 = 1200;
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
}

fn foobar(ind: usize) {
    let zeroes = [0, 0, 0, 0, 0];
    let f = zeroes[ind];
}
