#![allow(unused)]

use std::cmp;

/*
When using generics, rust does Monomorphization and fills in the concrete types for the generic types
to create new specific code.
Vec<T> when we use with i32 become Vec<i32>;

From the docs:
The compiler looks at all the places where generic code is called and
generates code for the concrete types the generic code is called with.
let integer = Some(5);
let float = Some(5.0);

Leads to the creation of concrete types like this:
enum Option_i32 {
    Some(i32),
    None,
}
enum Option_f64 {
    Some(f64),
    None,
}
 */

fn main() {
    let arr = vec![1, 5, 34, 50, 7, 25, 100, 65];
    // deref is implemented for Vec and returns an array
    let i = largest_int(&arr);
    println!("largest, first impl: {}", i);

    let p1 = Vector2 { x: 1, y: 2.0 };
    println!("{:?}", p1);

    let p2 = Vector2 { x: 10, y: 20 };
    println!("i32 Vector2 Op:{}", p2.weird_sum());

    println!("Genericed: {}", largest(&arr));
    struct Foobar;
    let p3 = Vector2 { x: 10, y: Foobar {} };
    let p4 = Vector2 { x: 16, y: Foobar {} };
    let has_smaller_x = p3.has_smaller_x(&p4);
    println!("Smaller x: {}", has_smaller_x);

    let p5 = Vector2 { x: Foobar, y: Foobar };
    // For this one, we can't call the `has_smaller_x` function since Foobar does not
    // impl PartialOrd
}

fn largest_int(slice: &[i32]) -> i32 {
    slice.iter()
        .fold(i32::MIN, |acc, cur| {
            cmp::max(acc, *cur)
        })
}

//pretty similar to java
// let's define this with type restrictions on T so we can
fn largest<T>(list: &[T]) -> T
    where T: PartialOrd + Copy {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Vector2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Vector2<T, U> {
    fn y(&self) -> &U {
        &self.y
    }
}

// We can also implement methods on a specific concrete types for T and U
impl Vector2<i32, i32> {
    fn weird_sum(&self) -> i32 {
        &self.y + &self.x
    }
}

impl<T, U> Vector2<T, U> {
    fn mixup_showcase<V, W>(self, that: Vector2<V, W>) -> Vector2<T, W> {
        Vector2 { x: self.x, y: that.y }
    }
}

// conditionally add an impl method to a struct
impl<T, U> Vector2<T, U> where T: PartialOrd {
    fn has_smaller_x(&self, that: &Vector2<T, U>) -> bool {
        &self.x < &that.x
    }
}

// the Try type from Scala as a generic algebraic type
enum Try<T, E> {
    Success(T),
    Failure(E),
}