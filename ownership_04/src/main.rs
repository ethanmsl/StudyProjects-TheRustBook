fn main() {
    let s_immut = String::from("hello");
    println!("{} world", s_immut);
    println!("{s_immut} world");
    // NOTE: using lsp to change variable name does *NOT* change the var
    //       inside the "{<var>}..." string embed.  This is an lsp shortcoming?

    let mut s_mut = String::from("hello");
    s_mut.push_str(", world");
    // I wish that made it clearer that it changed the value of its input (!)
    println!("{}", s_mut);

    let s1 = String::from("move me, baby!");
    let s2 = s1;
    //println!("s1 = {}", s1); // because s1 belongs to s2, println! can't have it!
    println!("s2 = {}", s2); // <-- fine! :)

    let s1b = String::from("copy me, baby!");
    let s2b = s1b.clone();
    println!("s1b = {}, s2b = {}", s1b, s2b);

    /*
    let x = String::from("hello");
    let mut y = x;
    y.push_str(" world");
    println!("{x}");
    */
    // Despite 'let x' being immutable "Copy" trait still isn't implemented
    // so movement of value still occurs.

    {
        let s = String::from("hello"); // s comes into scope
        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here
        let x = 5; // x comes into scope
        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.

    {
        let _s1 = gives_ownership(); // gives_ownership moves its return
                                     // value into s1
        let s2 = String::from("hello"); // s2 comes into scope

        let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.

    let outer_scope = String::from("move me to inner scope");
    {
        let inner_scope = outer_scope;
        // ^ this should be a move, causing the next line to fail:
        // println!("outer_scope after move to inner_scope: {}", outer_scope);
        println!("inner_scope = {}", inner_scope);
    }
    //println!("inner after inner_scope's scope collapses: {}", inner_scope);
    // ^ this doesn't work because inner_scope is gone
    // but *also*:
    //println!("outer_scope after inner_scope's scope collapses: {}", outer_scope);
    // ^ this also doesn't work
    // ownership did not transfer back after inner_scope disappeared

    let sab = String::from("hello");
    let bas = add_suffix(sab);
    //println!("{sab}");
    println!("{bas}");
    //sab = bas;
    // ^ not allowed because sab is immutable
}

fn add_suffix(mut s: String)->String {
    //         ^ this can take an immutable input and mutate it!!!
    s.push_str(" world");
    println!("internal add_suffix result: {}", s);
    s
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("some_string: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("some_integer: {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
