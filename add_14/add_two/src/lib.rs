use add_one::add_one;
pub fn add_two_fn(input: i32) -> i32 {
    add_one(add_one(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_two() {
        let result = add_two_fn(3);
        assert_eq!(result, 5);
    }
}
