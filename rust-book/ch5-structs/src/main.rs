#![allow(unused)]

fn main() {
    println!("Hello, world!");

    // create an instance of a Vector
    let unit = Vector3 { x: 1f64, y: 1f64, z: 1f64 };
    println!("({}, {}, {})", unit.x, unit.y, unit.z);

    // change a value of the mutable instance of the struct
    let mut position = Vector3 { x: 1f64, y: 1f64, z: 1f64 };
    position.z += 5 as f64;
    println!("({}, {}, {})", position.x, position.y, position.z);

    let (x, y, z) = (10.0, 5.0, 200f64);
    // since the variables we have here have the same name as the struct, we don't
    // need to repeat the names in the struct. EX:
    let far_vec = Vector3 { x, y, z };
    println!("({}, {}, {})", far_vec.x, far_vec.y, far_vec.z);

    // let's create a new vector with x being 7 but the other components from an existing vector
    let struct_update_vec = Vector3 { x: 56f64, ..far_vec };
    println!("({}, {}, {})", struct_update_vec.x, struct_update_vec.y, struct_update_vec.z);

    // create an instance of a Quaternion tuple struct
    let qt = Quaternion(0.23, 0.23, 0.1, 0.75);
    println!("qx: {}", qt.0);
    // we can destructure these as well using pattern matching
    let Quaternion(x, y, z, w) = qt;
    println!("Quaternion: x {} y {} z {} w {}", x, y, z, w);

    let rec1 = Rectangle { width: 10, height: 50 };
    // the curly braces work on types that implement display
    // using the format specified :? tells rust we want to use Debug format
    println!("{:?}", rec1);
    println!("Area Fucntion: {}", area(&rec1));

    // we can now use method syntax to call the area method on the Rectangle instance.
    println!("Area Method: {}", rec1.area());
    // Let's now call a method that takes ownership of self. our rec1 will no longer
    // be valid after this call
    let as_str = rec1.change_to_string();
    println!("Became Str: {}", as_str);

    // When calling methods, rust automatically creates the reference for us: automatic referencing and dereferencing
    // e.g. rec1.area() is the same as (&rec1).area();
    //-----


    // Let's call a function associated with the Rectangle struct
    let mk_square = Rectangle::square(30);
    println!("Became Str: {:?}", mk_square);
}

// we can add a trivial implementation of Debug using rust's derive attribute
// Traits that can be used with this derive attribute as listed
// at: https://doc.rust-lang.org/book/appendix-03-derivable-traits.html
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}

impl Rectangle {
    // this is called a method.
    // it's different from function in that it takes a reference to a `self` which
    //refers to a particular instance of the struct.
    // Note that methods can take ownership of instances or mutable borrow them just like any
    // other parameter. Example below
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn change_to_string(self) -> String {
        format!("Width:{}, Height: {}", self.width, self.height)
    }
}

// Also, we can have multiple impl blocks. Here we use that to separate the methods from
//associated functions.
impl Rectangle {
    // functions are defined in an impl block but don't take a self are not methods.
    // they're called, associated functions. These are associated with the struct and we
    // call them by using the struct name then :: and then the function name.
    fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

// For types where naming the fields would be redundant or just well known, let's use
// tuple structs.

struct Quaternion(f64, f64, f64, f64);

// unit like struct. Has no fields.
// useful if we want to implement methods attached to a type without
// storing any data.
struct Unit;

// if we want to store reference to data owned by something else, we have to use lifetimes.
struct User {
    username: String,
    // the struct here owns the String data for username
    email: String,
    sign_in_count: u64,
    active: bool,
}

/*
This would not work
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}
 */
// example with lifetime specifiers. More in chapter 10.
struct UserWLife<'a> {
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
    active: bool,
}

