fn main() {
    let s = String::from("hello world");
    let _s_literal = "hello world";
    // note that the first is a *String*
    // , but the second is a *&str* <-- an IMmutable reference
    // it is a slice of something stored (as constant, basically?)
    // in "the binary" <-- which doesn't quite makes sense in general

    // NOTE: [..5] == [0..5]
    let hello = &s[0..5];
    let world = &s[6..11];
    // NOTE: [6..] == [6..11] (for this string that has 11 elements)
    // NOTE: [..] == [0..11] (for this string that has 11 elements)

    println!("world = {world}");
    println!("hello = {hello}");


    // Array Slices
    let arr = [1,2,3,4,5];
    let arr_slice = &arr[1..3];
    assert_eq!(arr_slice, &[2,3]);
    // ^ panics if false
}


// example of trying to work with data minus slices
// using memory-independent, but conceptually-depenedent values
fn _first_word_raw(s: &String) -> usize /*index of word*/ {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; // if we find a ' '
        }
    }

    s.len() // if we don't find a ' '
}

// examply of above with string slices
fn _first_word_slice(s: &String) -> &str {
    //                  ^ can be generalized by use of '&str'
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // if we find a ' '
        }
    }

    &s[..] // if we don't find a ' '
}
