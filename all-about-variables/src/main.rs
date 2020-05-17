const FOO: f64 = 10e10 + 20f64;
const BAR: bool = true;

#[allow(unused_variables)]
fn main() {

    let mut x = 5;
    println!("The value of x is: {}", x);
    // let x:u64 = 0xFAFFABBF;
    let x: u64 = 0xFFF_AB_CDF_FAF_BFA;
    println!("The value of x is: {}", x);

    let x = 0b101101;
    println!("The value of x is: {}", x);

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The number of spaces is: {}", spaces);

    let tup = (50, 1100.023, "Jeff", true);

    println!("The tuple is: {:?}", tup);
    println!("Th values in ma tuple are: ({0}, {1})", tup.0, tup.1);

    let (first, floater, name, is_talking) = tup;
    println!("{}, {}, {}", first, floater, is_talking);


//    -------

    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let first = a[0];
    let second = a[1];
}
