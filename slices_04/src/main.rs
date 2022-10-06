fn main() {
    let s = String::from("hello world");

    // NOTE: [..5] == [0..5]
    let hello = &s[0..5];
    let world = &s[6..11];
    // NOTE: [6..] == [6..11] (for this string that has 11 elements)
    // NOTE: [..] == [0..11] (for this string that has 11 elements)

    println!("world = {world}");
    println!("hello = {hello}");
}


// example of trying to work with data minus slices
// using memory-independent, but conceptually-depenedent values
fn _first_word(s: &String) -> usize /*index of word*/ {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; // if we find a ' '
        }
    }

    s.len() // if we don't find a ' '
}
