//! # Doc comments
//! 
//! A library for showing how to use doc comments

// in lib.rs
pub mod compute;

/// Add one to the given value and return the value
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = hello_world::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/** Add two to the given value and return a new value

```
use hello_world::add_two;

let arg = 5;
let answer = add_two(arg);

assert_eq!(7, answer);
```
*/
pub fn add_two(x: i32) -> i32 {
    x + 2
}

/// Add three to the given value and return a [`Option`] type
pub fn add_three(x: i32) -> Option<i32> {
    Some(x + 3)
}


pub mod a {
    /// Add four to the given value and return a [`Option`] type
    /// [`hello_world::MySpecialFormatter`]
    pub fn add_four(x: i32) -> Option<i32> {
        Some(x + 4)
    }
}


