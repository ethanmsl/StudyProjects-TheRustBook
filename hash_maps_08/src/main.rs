fn main() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    // NOTE: no std method for constructing hashes
    //       unlike strings and vectors e.g. string::from("...") & vec!([...])

    // using .insert(k,v) to add mappings to the hashmap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
