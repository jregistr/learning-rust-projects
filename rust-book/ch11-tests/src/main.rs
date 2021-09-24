fn main() {
    println!("Hello, world!");
}

// ust builds a test runner binary that runs the functions annotated with the test attribute
#[test] // adding the test attribute turns a function into a test
fn foo() {
    println!("Hi");
}
