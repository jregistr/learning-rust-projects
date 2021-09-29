#![allow(unused)]

use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    let add_one = |x| x + 1;
    println!("Add One: {}", add_one(10));

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cached.value(intensity));
        println!("Next, do {} situps!", cached.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                cached.value(intensity)
            );
        }
    }
}

use std::collections::HashMap;

struct Cacher<T>
    where
        T: Fn(u32) -> u32
{
    func: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32
{
    fn new(func: T) -> Cacher<T> {
        Cacher { func, values: HashMap::new() }
    }

    fn value(&mut self, arg: u32) -> u32 {
        *self.values.entry(arg)
            .or_insert((self.func)(arg))
    }
}
