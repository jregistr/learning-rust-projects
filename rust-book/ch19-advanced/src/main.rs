#![allow(unused)]
mod avec;
use std::ops::Index;

static HELLO_WORLD: &str = "Hello World";

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // arbitrary memory address
    unsafe {
        *r2 += 10;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let r = &mut v[..2];
    r[0] = 10;
    println!("{:?}", r);
    println!("{:?}", v);

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    a[1] = 91;
    b[0] = 45;
    println!("{:?}", v);

    println!("impl it");
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let sl = &mut v[..];
    let (a, b) = split_at_mut(sl, 3);
    a[1] = 40;
    a[0] = 67;
    println!("{:?}", v);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);
    let raw_slice_ptr = slice.as_mut_ptr();

    unsafe {
        (
            std::slice::from_raw_parts_mut(raw_slice_ptr, mid),
            std::slice::from_raw_parts_mut(raw_slice_ptr.add(mid), len - mid)
        )
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Add<Point> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Point) -> Self::Output {
        Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z }
    }
}

impl std::ops::AddAssign<i32> for Vector3 {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::{Point, Vector3};

    #[test]
    fn add_two_points() {
        let a = Point { x: 1, y: 1 };
        let b = Point { x: 1, y: 1 };
        assert_eq!(Point { x: 2, y: 2 }, a + b)
    }

    #[test]
    fn add_point_to_vec() {
        let a = Vector3 { x: 1, y: 1, z: 1 };
        let p1 = Point { x: 1, y: 1 };
        assert_eq!(Vector3 { x: 2, y: 2, z: 1 }, a + p1);
    }

    #[test]
    fn add_5_to_vec() {
        let mut a = Vector3 { x: 1, y: 1, z: 1 };
        a += 5;
        assert_eq!(Vector3 { x: 6, y: 6, z: 6 }, a);
    }
}
