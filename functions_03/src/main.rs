fn main() {
    // NOTE: Haskell-like, functions don't have to be defined "above"
    // where they are called.
    // They just have to be defined somewhere such that, presumably
    // they are part of some index the compiler can use
    // e.g. here 'fiver()' & 'another_function()' are defined
    //      below 'main()'
    println!("Hello, world!");
    another_function(21, 'u');
    let fv = fiver();
    println!("The value of 'fiver' is: {fv}");
}

fn another_function(x: i32, y: char) {
    println!("Another function, whose values are {x} and {y}");
}

fn fiver() -> u8 {
    5
    // ^ note that we don't have an explicit 'return' statement
    //   just an unsilenced (un-`;`'ed) expression
}
