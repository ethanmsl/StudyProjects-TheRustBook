//! A file to work with recursive types using Box types
//! the particular recursive type of interest is what is known as a
//! **Cons List**, common in Lisp and apparently other functional programming languages
//! Seems to consist of pairs of pointers that generally point to a non-cons value
//! and another cons element
//!


/// A cons list -- recursive list type
///
/// # Conceptual Example
/// pseudo-code representation of a cons list containing 1, 2, and 3:  
///     `(1, (2, (3, (Nil))))`
///
/// # Verbiage Note:
/// `Nil` is the canonical name for the terminating or "base case" of at least some recursions
///
/// # Implementation Details:
/// ```
/// (i32, |ptr)
///       V
///      (i32, |ptr)
///            V
///           (i32, |ptr)
/// ```
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

// ^ coming back to this... WTF -- how is that where we define `Cons`
// how does the compiler know what this stuff is...
// ... I guess it's part of enum syntax 
//     -- arbitrary names for values, with optional types
