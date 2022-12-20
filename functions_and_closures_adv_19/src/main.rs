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

    // dynamic ~function generation via closures
    {
        let add_one = |x| x + 1;
        let add_forty_two = repeat(add_one, 42);
        println!(
            "Answer arrived at via closure composition is: {}",
            add_forty_two(0)
        );
    }
    println!("----------------------------------------\n");

    // functions as inputs, anonymous, named, derived
    {
        // with anonymous function
        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
        dbg!(list_of_strings);

        // with named function
        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> =
            list_of_numbers.iter().map(ToString::to_string).collect();
        dbg!(list_of_strings);

        // using the derived initializer function of an Enum
        #[derive(Debug)]
        enum Status {
            Value(u32),
            Stop,
        }
        let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
        // dbg!(list_of_statuses);  // <-- noisy
        println!("list_of_statuses: {:?}", list_of_statuses);
    }
    println!("----------------------------------------\n");

    // returning closures, which have no concrete type(!)
    {
        // /// won't compile
        // fn returns_closure_a() -> dyn Fn(i32) -> i32 {
        //     |x| x + 1
        // }

        /// *will* compile; what the compiler recommended when I gave it
        /// the above voce (`... -> **dyn** Fn(i32) -> i32`)
        /// apparently this implies **static dispatch**
        /// this want's something like a single concrete type at compile time
        /// but as closures nominally don't *have* concrete types it's unclear
        /// what the under the hood reqs are for this
        ///
        /// `impl trait` in an *input* parameter is an alias for giving a generic
        /// a trait-type
        /// e.g.
        /// `fn f(x: impl X) -> ...`
        ///  =
        /// `fn f<X: trait>(x: X) -> ...`
        ///
        /// ALSO usable as an *output* parameter e.g.
        /// `fn f() -> impl X`
        /// ^ in the above case, the `impl X` suggests that the specifics of what
        /// is being returned is **known at compile time** and has some
        /// "single concrete type" (though what that means for closures is not entirely clear)
        /// So taking two types that implement Display (e.g. char and i32)
        /// and trying to return either for a function that returns `impl Display`
        /// would **NOT** work
        fn returns_closure_b() -> impl Fn(i32) -> i32 {
            |x| x + 1
        }

        /// what the rust book recommended (...)
        /// `ptr<dyn trait>` nominally allows for dynamic elements of a trait to be
        /// returned, but it's not clear what that would mean with the signature
        /// being used here...
        fn returns_closure_c() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }

        // // `impl` in type alias blocks is an *un*stable feature
        // type Ci32x2 = impl Fn(i32) -> i32;
        // /// *will* compile; what the compiler recommended when I gave it
        // /// the above voce (`... -> **dyn** Fn(i32) -> i32`)
        // /// apparently this implies **static dispatch**
        // fn returns_closure_d() -> Ci32x2 {
        //     |x| x + 1
        // }
    }
    println!("----------------------------------------\n");

    // taking a look at `impl trait` in return type
    {
        // use std::fmt::Display;
        // /// will NOT compile
        // /// `impl trait` in return type requires a *specific* type
        // /// (probably not the best choice of syntax given that
        // /// it means something else in input type position)
        // /// ...
        // /// I think it's related to both resulting in monomorphization
        // /// ... but ... hmmm ... I still have questions about why trait-types
        // /// require dynamic dispatch in many cases still
        // /// as I'd think 
        // /// hm.... in my case below ... maybe it's related to the 
        // /// compiler wanting to know what it can plug into in a...
        // /// ...
        // /// lots tbd still
        // fn returns_displayable(choice: bool) -> impl Display {
        //     if choice {
        //         return 42;
        //     }
        //     'f'
        // }
    }
    println!("----------------------------------------\n");
}
