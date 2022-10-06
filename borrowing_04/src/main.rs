fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    ref_chain(&s1);

    let mut sm = String::from("welkom");
    let outp = change(&mut sm);
    println!("output of change() is : {outp}")
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

// apparently returning '&String' works
// , but NOT '&&&String',
// even though that's the nature of the variable I return
// ... I suppose it will allow a returning of a ref to s
// becuase s is a ref to something that exists
// outside the scope fo the function...
// is returning 'b: &&&String' and getting 'x(=b):&String'
// some special sauce here where they elide what's going on
// or some consistent system?
/*
fn ref_chain_broken(s: &String) -> &&&String {
    let a = &s;
    let b = &a;
    let c = &b;
    println!("from ref_chain: a,b,v : {a},{b},{c}");
    b
}
*/

fn change(some_string: &mut String) -> &String {
    some_string.push_str(", world");
    some_string
}
