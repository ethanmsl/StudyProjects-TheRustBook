//! A file to work with recursive types using Box types
//! the particular recursive type of interest is what is known as a
//! **Cons List**, common in Lisp and apparently other functional programming languages
//! Seems to consist of pairs of pointers that generally point to a non-cons value
//! and another cons element
//!
//! # Conceptual Example
//! pseudo-code representation of a cons list containing 1,2, and 3:
//!     `(1, (2, (3, (Nil))))`
//!
