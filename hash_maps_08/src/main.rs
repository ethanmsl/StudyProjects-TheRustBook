//don't warn about unused code
// #![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    // NOTE: no std method for constructing hashes
    //       unlike strings and vectors e.g. string::from("...") & vec!([...])

    // using .insert(k,v) to add mappings to the hashmap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    // accessing values from a hashmap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    //                 ^ get ref to val
    //                                  ^ copy ref's data
    //                                           ^ extract the T from the Some(T)
    println!("score: {score}");

    // looping oveer key:value pairs in a hashmap
    // Note: looping order is ambiguous to the programmer
    for (key, value) in &scores {
        println!("key: {}, value: {}", key, value);
    }
}
