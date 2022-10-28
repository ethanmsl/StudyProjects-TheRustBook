pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// a nice little testing module
// that's created by default on `cargo new --lib` ! :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
