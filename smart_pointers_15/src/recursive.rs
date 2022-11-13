//! A file to work with recursive types using Box types
//! the particular recursive type of interest is what is known as a
//! **Cons List**, common in Lisp and apparently other functional programming languages
//! Seems to consist of pairs of pointers that generally point to a non-cons value
//! and another cons element
//!


/// A cons list -- recursive list type
///
/// # Conceptual Example
/// pseudo-code representation of a cons list containing 1,2, and 3:
///     `(1, (2, (3, (Nil))))`
///
/// # Verbiage Note:
/// `Nil` is the canonical name for the terminating or "base case" of at least some recursions
enum List {
    Cons(i32, List),
    Nil,
}
// NOTE: ^ The above is an example of a NON-compiling attempt at a recursive type
