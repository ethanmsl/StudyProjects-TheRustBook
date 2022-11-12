/// does this produce documentation ?
/// *yes*?
/// **no**?
/// ~~maybe~~?
/// ANSWER: no
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
