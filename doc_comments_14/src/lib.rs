//! # doc_comments_14 -- lib.rs file
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient./// does this produce documentation ?
//! this is in lib.rs
//! ```
//! assert_eq!(6,6);
//! ```
//  ^ NOTE: that test *is* run

/// *yes*?
/// **no**?
/// ~~maybe~~?
/// ANSWER: Yes
pub const GREETING: &str = "Hallo, Rust library housed const here!";

/// Adds one to the number given.
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = doc_comments_14::add_one_lib(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one_lib(x: i32) -> i32 {
    x + 1
}

/// While this prints to screen, it's really just here to write aboud and use doc-comments
///
/// # Common Sections:
/// ## Examples
/// ## Panics
/// ## Errors
/// ## Safety
pub fn just_for_the_docs_lib() {
    println!("This is just for the docs");
}

/// Adds one to the number given.
///
/// # Examples
/// ```
/// let arg = 5
/// let answer = doc_comments_14::add_two(arg);
///
/// assert_eq!(7, answer);
/// ```
/// NOTE: that ^ test won't be run by doc tests because this function is in main.rs!!!!
fn add_two_main(x: i32) -> i32 {
    x + 1
}
// NOTE: ^ Private
