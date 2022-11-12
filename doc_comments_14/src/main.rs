use doc_comments_14::GREETING;

fn main() {
    println!("{}", GREETING);
}

/// Adds one to the number given.
///
/// # Examples
/// ```
/// let arg = 5
/// let answer = doc_comments_14::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
/// NOTE: that ^ test won't be run by doc tests because this function is in main.rs!!!!
pub fn add_one_main(x: i32) -> i32 {
    x + 1
}

/// While this prints to screen, it's really just here to write aboud and use doc-comments
///
/// # Common Sections:
/// ## Examples
/// ## Panics
/// ## Errors
/// ## Safety
pub fn just_for_the_docs_main() {
    println!("This is just for the docs");
}
