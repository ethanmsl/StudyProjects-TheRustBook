//prevent error warnings
// #![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
// #![allow(dead_code)]
fn main() {
    let mut s = String::new();

    // (1)
    let data = "initial contents";
    let s = data.to_string();

    // (2) the method also works on a literal directly:
    let s = "initial contents".to_string();

    // (3)
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    // ^ note that s2 was not taken by s1

    let mut s = String::from("lo");
    s.push('l');
    // ^ 'push' takes a char, unlike 'push_str'

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // ^ note s1 (but not s2) has been moved here and can no longer be used
    println!("s3 is {s3}, s2 is {s2}, s1 is {{just kidding, s3 took ownership of it}}");
    // note '+' uses 'add(...)' which takes an ownership and a reference
    // it's not a matter of choice with that operator
    // (though it seems to allow either order for the '+' version)

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);
}
