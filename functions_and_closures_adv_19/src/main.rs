//! Advanced Functions and Closures -- Advanced Features Ch 19

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}

// /// not valid
// /// the `where` syntax seems to be for traits only
// fn do_thrice(f: F, arg: i32) -> i32
// where
//     F = Fn(i32) -> i32,
// {
//     f(f(f(arg)))
// }

type I32x2 = fn(i32) -> i32;

fn do_thrice(f: I32x2, arg: i32) -> i32
{
    f(f(f(arg)))
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let answer = do_thrice(add_one, 5);
    println!("The answer is: {}", answer);

}
