#![allow(unused)]

use std::fmt::Display;

fn main() {
    // Vectors: A growable list of items of the same type.
    let v: Vec<i32> = Vec::new();
    // we can also create one without adding the type annotation. The
    // compiler can infer it when we add the first item
    let mut v = Vec::new();
    v.push(10);

    // to create a vector with initial values, use the vec! macro
    let v = vec![1, 2, 3, 4];
    println!("{:?}", v);
    let t = v[2];
    let t_ref = &v[2];
    println!("third value: {}", t);

    // Access Failure:
    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100]; // will panic due to out of bounds error
    let does_not_exist = v.get(100); // this one returns an Option type.


    /*
    // borrow example
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // we borrow an element in the vector here.
    v.push(6); // and here we try to mutate the vector by doing a mutable borrow
    println!("The first element is: {}", first); // we use the immutable borrow here
    // Hence, an immutable & mutable borrow exist at the same time
     */

    // iterating the vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    //iterating the vector and changing values
    let mut v = v;
    for i in &mut v {
        *i *= 10;
    }
    println!("{:?}", v);

    // Can we iterate and use an index to change the array?
    for index in 0..v.len() {
        v[index] *= 5;
    }

    /*
    This won't work since calling iter is doing an immutable borrow
    println!("{:?}", v);
    let mut iter = v.iter();
    for index in 0..v.len() {
        let a = iter.next().unwrap();
        v[index] *= a;
    }
     */

    strings_notes();
    indexing_into_strings();
    do_hashmap();
}

fn strings_notes() {
    println!("----- Strings --------");
    let mut a = String::from("Здравствуйте");
    a += "!!";
    a.push_str(" Hello");
    println!("{}", a);

    let use_format = format!("{} {} {}", String::from("tic"), "tac", a);
    println!("{}", use_format);

    let s1 = String::from("S1");
    let s2 = "second";
    let s3 = s1 + &s2;
    // the signature of the add function is fn add(self, s: &str)
    // println!("{}", s1); //thus s1 is now invalid
    // so we've moving s1 to the add function, and then making a copy of &s2 to the s1 to make
    // a new string.
}

fn indexing_into_strings() {
    println!("---------- Indexing ---------------");
    // internally, Strings in rust is a wrapper around a vector of u8's(byte).
    // the letter 'a' is represented by 1 byte. but cyrillic character 'й' is represented by two bytes.
    let hindi = String::from("नमस्ते");
    println!("{:?}", hindi.chars());
}

fn do_hashmap() {
    println!("-------------- HashMap ---------------");
    use std::collections::HashMap;

    let mut car_reviews = HashMap::new();
    car_reviews.insert("BMW i3".to_string(), "Trash!! Terrible, don't buy it!".to_string());

    // creating a map from list of tuples
    let v = vec![(String::from("Any BMW"), 4), (String::from("Toyota Prius"), 7)];
    let b = v.into_iter().collect::<HashMap<String, i32>>();
    // let b: HashMap<String, i32> = v.into_iter().collect();
    // both approaches are valid
    println!("{:?}", b);

    let maybe_review = b.get("Any BMW"); // returns an Option type
    let def_review = b["Any BMW"]; // this one gets the value and will panic if it doesn't exist in the map.

    for(k, v) in &b {
        println!("{}", k);
    }

    let mut b = b;
    b.entry(String::from("Any BMW")).or_insert(10); // just no
    b.entry(String::from("Any Jeeps")).or_insert(6);
    // the or_insert function returns a mutable reference to the value inserted or existing

    *b.entry(String::from("Any Jeeps")).or_insert(7) = 6;
    // set it to 7 then follow the pointer and set it back to 6

    // let's add 1 to a value or set 1 if it does not exist using the entry
    *b.entry(String::from("VW")).or_insert(0) += 1;

    println!("{:?}", b);
}
