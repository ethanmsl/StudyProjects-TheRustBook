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

    // WARNING: copy vs move of values inserted into HashMap requires awareness

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value have moved their values to map
    // // the following code will create an error:
    // println!("field_name: {}, field_value: {}", field_name, field_value);


    // ------------- Ovewriting a value -------------
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("scores: {:?}", scores);
    scores.insert(String::from("Blue"), 25);
    println!("scores: {:?}", scores);
    println!("------------------------------");

    // ------------- Conditional adding of a key:value -------------
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("scores: {:?}", scores);
    
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("scores: {:?}", scores);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores: {:?}", scores);
    println!("------------------------------");

    // ------------- Updating a value conditioned on old value -------------
    let text = "hello world wonderful world";
    //                     ^ lack of ',' intended
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("map: {:?}", map);
    println!("------------------------------");


    // NOTE: `SipHash` is apparently the default hashing algorithm
    //      it trades some speed for anti-DoS security
    //      other "hasher"s an be specified

}
