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

    let v2= vec![1, 2, 3];
    let v2_iter = v2.iter();
    let boop = v2_iter.map(|x| x+1);
    for elem in boop {
        println!("boop: {}", elem);
    }
    // // Won't work because v2_iter was taken by .map(...)
    // for elem in v2_iter {
    //     println!("v2: {}", elem);
    // }
    let v22_iter = v2.iter();
    for elem in v22_iter {
        println!("v22: {}", elem);
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

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn collect_behavior() {
        let v1 = vec![1, 2, 3,];
        let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
        //        ^ this type annotation was needed

        assert_eq!(v2, vec![2, 3, 4,]);
    }
}
