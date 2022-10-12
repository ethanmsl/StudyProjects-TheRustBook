fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    ref_chain(&s1);

    let mut sm = String::from("welkom");
    let outp = change(&mut sm);
    println!("output of change() is : {outp}");

    // -----------------------------------------
    // NLL: Non-Lexical Lifetimes
    //      r3 can take write access to value
    //      even though read access was had in the
    //      formal scope
    //      as the compiler can tell that the read references
    //      are no longer used at granting of write reference
    let mut r_source = String::from("hello");

    // these two read-refs are fine
    let r1 = &r_source;
    let r2 = &r_source;
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    // this write-ref is fine *because* rust detects that the
    // r1, r2 refs had their final use *prior* to it
    let r3 = &mut r_source;
    println!("{}", r3);

    // ---------------------------------------
    // Implicit Borrowing
    let mut em_source = String::from("explicit move");
    let em2 = &mut em_source;
    let _em3 = em2;
    // println!("em2 = {em2}")
    // ^ would not be valid as em2 has 'moved'
    let mut ib_source = String::from("implicit borrow");
    let ib2 = &mut ib_source;
    calculate_length(ib2);
    // implicitely becomes : 'consume(&mut *s2)' ... apparently
    println!("ib2 = {ib2}");
    // ^ valid because ib2 is NOT 'moved'
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
