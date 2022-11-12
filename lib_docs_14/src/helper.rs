//! I'm a helper file
//! ================
//! do I get documentation too?!
//!

/// I'm a public helper FUNCTION ( () )
///
/// # Examples
///
/// ```
/// println!("Hello, world!");
/// assert_eq!(1, 1);
/// ```
pub fn helper_pub() {
    helper_priv();
}


/// I'm a private helper FUNCTION ( () )
///
/// # Examples
///
/// ```
/// println!("Hello, world!");
/// assert_eq!(1, 1);
/// ```
fn helper_priv() {
    println!("I'm a helper function");
}

