#![allow(dead_code)]
fn main() {
    println!("The src file is mostly unused functions and tests.");
}

/////////////////// to compare output during tests //////////////////
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value: {}", a);
    10
}

//////////////////// choosing specific tests to run //////////////////
pub fn add_two(a: i32) -> i32 {
    a + 2
}

///////////////// TESTS //////////////////
#[cfg(test)]
mod tests {
    use super::*;

    //////////////////// choosing specific tests to run //////////////////
    // Run tests containing phrase: `cargo test phrase`
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    // /////////////////// to compare output during tests //////////////////
    // // Show Output even if Successful: `cargo test -- --show-output`
    // #[test]
    // fn this_test_will_pass() {
    //     let value = prints_and_returns_10(4);
    //     assert_eq!(10, value);
    // }
    //
    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }
}
