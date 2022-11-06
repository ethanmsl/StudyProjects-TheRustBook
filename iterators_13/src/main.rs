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

    for val in v1_iter {
        println!("Got: {}", val);
    }
}
