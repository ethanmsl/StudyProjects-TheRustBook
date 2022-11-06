// public
pub fn add_two(a: i32) -> i32 {
    internal_adder(a,2)
}

// private
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // tests module can access the private function becuase
    // *universally*: child modules can use all of ancestor modules' functions
    fn internal() {
        assert_eq!(4, internal_adder(2,2));
    }
}
