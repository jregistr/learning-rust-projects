/*
    {
        let r;                // ---------+-- 'a
        //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
        //          |
        println!("r: {}", r); //          |
    }                         // ---------+
Here, r has lifetime 'a and x in the inner scope block has lifetime 'b. The lifetime of 'a is larger
than 'b therefore, r referencing x will lead to pointing to data that is no longer valid.
*/

/*
fn main() {
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
}
Conversely, r has a smaller lifetime 'a than x 'b. Because the item with the smaller lifetime references
data with a larger lifetime, at no point during its lifetime will the data go out of scope.
 */

/*
Here's another example but with functions. Let's say we have a function that returns the longest of two
string slices. It's possible that we accidentally pass it two string slices where one outlives the other.
Rust will not allow this to compile.
EX:
// this won't compile
fn longest(str1: &str, str2: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
Here's an example proving this.
let x = "i am a 1";
let z;
{
    let y = "i too am 1 but bigger";
    z = longest(x, y);
}
 */


use std::fmt::Display;

// To make this work
// let's annotate the longest function with lifetime annotations
// here we label 1 lifetime a and we specify that for a lifetime 'a, we have two parameters
// and the return value that live at least as long as 'a. In practice, this means that the return value with lifetime 'a will
// as long as the smaller of the two lifetimes of x and y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
When returning a reference from a function, the lifetime parameter for the return
type needs to match the lifetime parameter for one of the parameters.
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proper_use() {
        let s1 = "i am a 1";
        {
            let s2 = "i too am a 1 but bigger";
            let res = longest(s1, s2);
            println!("The longest is: {}", res);
        }
    }
}

/*
When defining structs that use references, we need to attach a lifetime annotation to each reference field.
Here, we say that the lifetime of Excerpt can't outlive text.
 */
#[derive(Debug)]
struct Excerpt<'a> {
    text: &'a str,
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn from_novel() {
        let novel = String::from("Thus, I think, therefore, I am. This is a legend.");
        let quote = novel.split('.').next().expect("No sentences");
        let excerpt = Excerpt {
            text: quote
        };
        println!("{:?}", excerpt);
    }
}

/*
Lifetime Elision rules
fn first_word(s: &str) -> &str {} <- lifetime annotations were not needed here because this is a pattern added to the compiler.
If the code fits the Lifetime Elision rule cases, then lifetime annotations will not be needed

input lifetimes - Lifetimes on function or method parameters
output lifetimes - Lifetimes on return values

Rule 1:
The first rule is that each parameter that is a reference gets its own lifetime parameter.
fn foo<'a>(x: &'a i32)   or fn foo<'a, 'b>(x: &'a i32, y: &'b i32)

Rule 2:
The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
fn foo<'a>(x: &'a i32) -> &'a i32

Rule 3:
The third rule is if there are multiple input lifetime parameters,
but one of them is &self or &mut self because this is a method. The lifetime of self is assigned to all output lifetime parameters.
---------------

EX 1:
fn first_word(s: &str) -> &str { }
The first rule is applied: each param gets its own lifetime
fn first_word<'a>(s: &'a str) -> &str { }

There is exactly one lifetime so second rule applies and a lifetime is added to the output type.
fn first_word<'a>(s: &'a str) -> &'a str { }
---------

EX 2:
fn longest(x: &str, y: &str) -> &str {}
Apply the first rule and place a unique lifetime for each parameter
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}

X - The second rule does not apply since there are more than 1 lifetimes
X - The third rule does not apply since one of the inputs isn't self.
---------

EX 3:
fn validate_review(&self, contains_word: &str) -> &str {}

Apply the first rule and place unique lifetime for each parameter.
fn validate_review<'a, 'b>(&'a self, contains_word: &'b str) -> &str {}

X - The second rule doesn't apply. There are more than 1 lifetimes

Y - The third rule does apply because one of the reference parameters is self. Thus, apply the lifetime
of self to the output.
fn validate_review<'a, 'b>(&'a self, contains_word: &'b str) -> &'a str {}
*/

struct Food {
    review: String,
}

impl Food {
    fn validate_review(&self, contains_word: &str) -> &str {
        if self.review.contains(contains_word) {
            &self.review
        } else {
            &self.review[..4]
        }
    }
}

// ------------------
/*
In impl blocks, lifetime parameters are always needed if the type uses them. Ex: below
*/
#[derive(Debug)]
struct Quote<'a> {
    text: &'a str,
}

impl<'a> Quote<'a> {
    // The first rule applies here. But the output is not
    // a reference so it's unchanged.
    // our signature becomes: fn level<'a>(&'a self) -> i32 {}
    fn level(&self) -> i32 {
        3
    }

    // The third rule applies here.
    // fn say<'a, 'b>(&'a self, to_say: &'b str) -> &'a str { }
    fn say(&self, to_say: &str) -> &str {
        println!("Thing to say {}", to_say);
        self.text
    }
}

// All string literals implicitly have the 'static lifetime.
/*
let s: &'static str = "I have a static lifetime.";
is the same as
let s: &str = "I have a static lifetime.";

This lifetime means the reference can live the entire duration of the program.
*/

// Now put it all together. Here's a function making a generic announcement before returning the largest of two ref strs.
fn say_something_get_largest<'a, T: Display>(str1: &'a str, str2: &'a str, to_say: T) -> &'a str {
    println!("Attention everyone, it's time to say: {}", to_say);
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

#[cfg(test)]
mod tests3 {
    use super::*;

    #[test]
    fn say_and_get() {
        let q1 = "this is a quote. There are many like it";
        let q2 = "but this one is mine";
        let to_say = String::from("Turn the mic up!");
        say_something_get_largest(q1, q2, to_say);
    }
}

