//! A Test of Whether Docs are Produced for Binary (main.rs only) Files
//! ================================================================
//! # How are you?
//! - I'm fine, thanks.
//!  - How are you?
//!   - Also fine, thanks.
//! # **NOTE**: Still no tests run, however!
//! There seem to be reasons that, as is, rust doesn't feel able to run tests
//! in the binary  
//! ... and this is why functionality is often pulled out to lib.rs  
//! ... but then it jeems strange that you can doc main.rs unless there exists a lib.rs  
//!
//! ... I need to test how other files affect this

mod helper;

/// The main function.
/// # Examples
///
/// ```
/// println!("Hello, world!");
/// ```
fn main() {
    println!("Hello, world!");
    println!(" 2147483647 * 6 = {}", a_mult_b(2147483647, 6));
    helper::helper_pub();
}


/// Multiplies two numbers.
///
/// # Examples
/// ```
/// let x = 2147483647;
/// let y = 6;
/// assert_eq!(-6, multiply(x, y));
/// ```
///
/// # Panics
/// This function will panic if the result overflows a `i32`.
///  ^ **NOTE**: I don't know if that's true, that was co-pilot auto-generated
/// 
/// As suspected/dimly remembered:
/// it causes a panic **in dev mode**
/// , but mereley 'wraps' in release mode.
/// e.g. `cargo run` panics (as we do the above example in main()
/// , but `cargo run --release` does not. (and instead procduces `-6`)
///
/// # **NOTE**: Still no tests run, however!
/// There seem to be reasons that, as is, rust doesn't feel able to run tests
/// in the binary
/// ... and this is why functionality is often pulled out to lib.rs
/// ... but then it seems strange that you can doc main.rs unless there exists a lib.rs
///
/// ... I need to test how other files affect this

fn a_mult_b(a: i32, b: i32) -> i32 {
    a * b
}
