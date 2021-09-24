#![allow(unused)]

mod slice;

use std::fmt::{Debug, Formatter};

fn main() {
    let f = String::from("hi");

    /*
    Won't compile because the string was moved to s2 and then println tries to borrow it
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);
    */

    /*
    let v1 = Vector {x: 1, y: 1, z: 1};
    let v2 = v1;
    println!("{:?}", v1);
    // This one works because Vector implements Copy
     */

    // Similar to the Vector struct above, i32 implements Copy
    let i1 = 1;
    let i2 = i1;
    println!("{}", i1);

    let v = make_a_vec();
    // Our function here takes ownership of the vector so v would no longer be valid
    // but this function returns it in a tuple so we can gain ownership back.
    let (v, magnitude) = take_vector_and_give_back(v);
    println!("Magnitude of {:?} is {:.2}", v, magnitude);

    // let's instead pass a reference to our vector since the function really does not
    // need to own the vector to calculate its magnitude.
    let magnitude = calculate_length(&v);
    // we can use use the vector since v is still valid
    println!("Magnitude of {:?} is {:.2}", v, magnitude);

    let mut st_ex = String::from("Well, hello");
    // here we pass a mutable reference to our mutable value
    // note we can only have 1 mutable borrows at a time.
    // if we have no mutable borrows, then we can have infinite immutable borrows
    change(&mut st_ex);
    println!("{}", st_ex);


    // ----------------------------------------------
    // ----------------------------------------------
    // More on mutable and immutable references
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    // println!("{}", r1); Not Okay!
    /*
    Because r1 and r2 are not used after the println statement, their scope end. So its fine
    that the r3 mutable ref is created. If we did however, use r1 or r2 after the r3 mutable ref was
    created, then they would both be existing at the same time.
     */
    println!("----- moving to slices ---------");

    let mut s = "well hello world".to_string();
    let sliced = slice::first_word(&s);
    // s.clear(); This is a mutable borrow, so if we try to use sliced, we'd be using an immutable
    // borrow at the same time we have a mutable borrow.
    println!("{}", sliced);

    let s = "well hello world".to_string();
    // we can pass a reference to a string to a ref str
    // does not work the other way around though
    // this takes advantage of implicit deref
    println!("Second Try: {}", slice::first_word_ref_str(&s));

    let ar = [1,2, 3,4, 4];
    let ar_slice = &ar[..3];
}

#[derive(Debug, Copy, Clone)]
struct Vector {
    x: i32,
    y: i32,
    z: i32
}

fn make_a_vec() -> Vector {
    Vector{x: 1, y: 1, z: 1}
}

fn take_vector_and_give_back(v: Vector) -> (Vector, f64) {
    let mag = v.x.pow(2) + v.y.pow(2) + v.z.pow(2);
    let mag = (mag as f64).sqrt();
    (v, mag)
}

// Takes a reference to a Vector rather than taking ownership
fn calculate_length(v: &Vector) -> f64 {
    let mag = v.x.pow(2) + v.y.pow(2) + v.z.pow(2);
    (mag as f64).sqrt()
    // v goes out of scope here, but because it does not own the data, the data does not
    // get dropped
}

/*
fn change(some_string: &String) {
    some_string.push_str(", world");
}
this function will not compile. We can't modify a reference. by default, these are immutable.
 */


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
