pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // format!("Hello {}!", "boop")
    // // ^ to break the 'greeting_contains_name()' test
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // guardian: filters values
        if !(1..=100).contains(&value) {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        // uses values that passed guard to populate Guess
        Guess { value }
    }
}

//////////////////////////////////// TESTS ////////////////////////////////////
// a nice little testing module
// that's created by default on `cargo new --lib` ! :)
#[cfg(test)]
mod tests {
    use super::*;

    ///////////////////// Guess_tests /////////////////////
    #[test]
    #[should_panic(expected = "should be within [1,100]")]
    // #[should_panic]
    // ^ this is what allows our panic to result in a test pass, it seems
    fn greater_than_100() {
        Guess::new(200);
    }

    //////////////////// greeting_tests ////////////////////

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, output value was '{}'",
            result
        );
    }

    //////////////////// rectangle_tests ////////////////////
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    //////////////////// general_tests ////////////////////
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does nto equal four"))
        }
    }

    #[test]
    fn it_shouldnt_work() {
        let result = add(2, 2);
        assert_ne!(result, 7982);
    }
}
