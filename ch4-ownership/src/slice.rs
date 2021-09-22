/*
A slice references a contiguous sub sequence of elements in a collection rather than the whole collection.
A slice does not own the data.

Let's write a program that takes a string and returns the first word in that string. Should their be
no spaces, than the whole string is one word. What should be the return type.
 */


//Approach 1: return the index of the end of the first word
/*
This solution is flawed. The owner of the string can later change the string for example by clearing
all bytes in it. Our usize would no longer be valid after that so if we use it to traverse
the string, we could cause a runtime error.
let mut s = String::from("xdd dd");
let word_end = first_word(&s);
s.clear();

The issue at hand is that we have a value that was created from the state of an object but is not
tied to that state at all.
 */
fn first_word_bad(s: &String) -> usize {
    let bytes = s.as_bytes();

    // iter will give us reference to elements so we use &
    for (i, &item) in bytes.iter().enumerate() {
        // this b is the byte literal syntax
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

/*
Rust solves this problem by providing us the string slice type.
The syntax looks like this.
let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];


    Range syntax
    [starting_index..ending_index (non-inclusive)]
    if we want to start at 0, then we can drop the starting_index
    [..5] is equivalent to [0..5]
    If our slice includes the last element in the collection, we can skip adding it
    [3..s.length] is equivalent to [3..]
    Lastly, if we want a slice of the whole string, we can ommit both starting and ending index
    [0..s.length] is equivalent to [..]
 */
// Let's solve the problem again but using a string slice
pub fn first_word(s: &String) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

pub fn first_word_ref_str(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
