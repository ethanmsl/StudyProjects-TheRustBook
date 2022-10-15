#![allow(unused_variables)]
fn main() {
// ------ calling panic!(...) macro ------
    // panic!("crash and burn");
    // // will print:
    // // thread 'main' panicked at 'crash and burn', src/main.rs:4:5
    // // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// ------ panicking by non-valid array call ------
    let v = vec![1, 2, 3];
    // v[99];
    // // will print:
    // // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:10:5
    // // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

}
