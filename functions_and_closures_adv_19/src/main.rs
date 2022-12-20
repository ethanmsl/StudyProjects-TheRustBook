//! Advanced Functions and Closures -- Advanced Features Ch 19

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}

// /// not valid
// /// the `where` syntax seems to be for traits only
// /// one of the compiler errors that comes up is:
// /// "equality constraints are not yet supported in `where` clauses"
// /// the "yet" being, hopefully, noteworthy :)
// fn do_thrice(f: F, arg: i32) -> i32
// where
//     F = fn(i32) -> i32,
// {
//     f(f(f(arg)))
// }

type I32x2 = fn(i32) -> i32;

fn do_thrice(f: I32x2, arg: i32) -> i32 {
    f(f(f(arg)))
}

#[allow(dead_code)]
/// # Doesn't work
/// I've heard told that Rust doesn't allow for static function composition
/// (though composition via closures seems possible)
fn emit_repeated_function(f: I32x2, reps: u32) -> I32x2 {
    if reps == 0 {
        return |x: i32| x;
        //     ^ this should be courced to `fn` as it does not capture anything
    }

    // reps >= 1
    #[allow(unused_mut)]
    let mut func = f; // note that this is the first rep, itself
    for _ in 1..reps {
        // func = f(func);
        // func = f(func(x: i32));
        // func = f(func(<i32>));
        // func = |x| f(func(x));
        // ...?
    }
    func
}

/// Does not *emit* the function we want, but rather performs *as* the function
fn be_repeated_function(f: I32x2, reps: u32, inp: i32) -> i32 {
    let mut val = inp;
    for _ in 0..reps {
        val = f(val);
    }
    val
}

/// Returns a *closure* rather than a function, but the closure is within `Fn`(not `fn`)
/// But, does do the dynamic composition we were looking for
fn repeat(f: impl Fn(i32) -> i32, reps: usize) -> impl Fn(i32) -> i32 {
    move |x| {
        let mut current = x;
        for _ in 0..reps {
            current = f(current);
        }
        current
    }
}

fn main() {
    println!("----------------------------------------\n");
    {
        let answer = do_twice(add_one, 5);
        println!("The answer is: {}", answer);

        let answer = do_thrice(add_one, 5);
        println!("The answer is: {}", answer);

        // using repeating function
        let answer = be_repeated_function(add_one, 1, 5);
        println!("The answer is: {}", answer);

        let answer = be_repeated_function(add_one, 2, 5);
        println!("The answer is: {}", answer);

        let answer = be_repeated_function(add_one, 3, 5);
        println!("The answer is: {}", answer);
    }
    println!("----------------------------------------\n");

    {
        let add_one = |x| x + 1;
        let add_forty_two = repeat(add_one, 42);
        println!(
            "Answer arrived at via closure composition is: {}",
            add_forty_two(0)
        );
    }
    println!("----------------------------------------\n");
}
