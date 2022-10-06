fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    ref_chain(&s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s going out of scope returns full ownership to original
  // similar to what we thought move would do, but with the important
  // adjective "full" on ownership, as read-only references can still
  // be passed from the original
  //
  // Q: can I pass references in a chain?
fn ref_chain(s: &String) -> &String {
    let a = &s;
    let b = &a;
    let c = &b;
    println!("from ref_chain: a,b,v : {a},{b},{c}");
    b
}
