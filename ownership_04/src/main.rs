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
    //println!("{}", s1); // because s1 belongs to s2, println! can't have it!
    println!("{}", s2); // <-- fine! :)
}
