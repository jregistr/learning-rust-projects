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
}
