#![allow(unused)]
mod lifetimes;

use std::fmt::{Display, Debug};
use std::iter::Sum;

pub fn notify(item: &impl Summary) {
    println!("Well... Breaking NEWS!! {}", item.summarize());
}

// with this approach, item1 and item2 can be different types, so long as they both
// implement Summary
pub fn notify_two(item1: &impl Summary, item2: &impl Summary) {}

// to force both item1 and item2 to be the same type, we have to use the trait bounds syntax
pub fn notify_two_bound<T: Summary>(item1: &T, item2: &T) {}

pub fn notify_bounds<T: Summary>(item: &T) {
    println!("Well... Breaking NEWS!! {}", item.summarize());
}

// here, we specify multiple traits we require using +
pub fn notice_and_display(item: &(impl Summary + Display)) {}

// this same plus syntax applies to trait bounds syntax
pub fn notice_and_display_bound<T: Summary + Display>(item: &T) {}

// lastly, if we have many trait restrictions, it can make the function hard to read
// by having so many trait bounds between the function name and its params list. e.g. below
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { todo!() }

// let's use rust's `where` clause syntax
fn some_other_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone, U: Clone + Debug {
    todo!()
}


// we can use an impl trait as a return value. but this only work if in all cases,
// our function is only returning one concrete type.
fn mk_summarizable() -> impl Summary {
    Tweet { content: "l".to_string(), has_retweets: true, user: "m".to_string() }
}

// this would however not be valid
/*
fn mk_mk_summarizable_2(test: bool) -> impl Summary {
    if(test) {
        Article {...}
    } else {
        Tweet {}
    }
}
 */

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Article {
    pub headline: String,
    pub author: String,
}

pub struct Tweet {
    pub has_retweets: bool,
    pub user: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} Written by {}", &self.headline, &self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} chirped by user {} and has {} retweets",
                &self.content,
                &self.user,
                if self.has_retweets { "" } else { "0" }
        )
    }
}

/*One restriction to note with trait implementations is that we can implement a trait
on a type only if either the trait or the type is local to our crate.*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarize_tweet() {
        let tweet = Tweet { has_retweets: true, user: String::from("u2"), content: String::from("such tweet") };
        println!("Tweet Summary: {}", tweet.summarize());
        notify(&tweet);
        notify_bounds(&tweet);
    }
}
