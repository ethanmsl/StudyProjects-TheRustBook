// // Iterator trait written out for easy reference
// pub trait Iterator {
//     type Item;
//
//     fn next(&mut self) -> Option<Self::Item>;
//     // "methods with default implementations elided"
// }

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    //  ^ NOTE: that while immutable, when used in the for loop below
    //          ownership is given tot the for loop 
    //          and mutability is effectively declared there

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterstor_demonstration() {
        let v1 = vec![1, 2, 3,];

        let mut v1_iter = v1.iter();
        //   ^ NOTE: mutable, needed to manually use .next()

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None    );
    }
}
