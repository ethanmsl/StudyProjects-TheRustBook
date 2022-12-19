//! Advanced Types - Advanced Features Ch. 19

#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    println!("---------------------------------------------\n");

    // *******Type Alias********
    // merely syntactic convenience
    // a **single** type with an extra syntactic-handle
    // (useful if a type's default name is unwieldly)
    {
        type Kilometers = i32;

        let x: i32 = 5;
        let y: Kilometers = 5;

        println!("x(raw_i32) + y(km) = {}", x + y);

        /// example unwieldly type being used
        /// (apparently "thunk" is a term for code to be evaluated later)
        type Thunk = Box<dyn Fn() + Send + 'static>;

        let f1: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
        let f2: Thunk = Box::new(|| println!("hi"));

        fn takes_long_type_1(f: Box<dyn Fn() + Send + 'static>) {}
        fn takes_long_type_2(f: Thunk) {}

        fn returns_long_type_1() -> Box<dyn Fn() + Send + 'static> {
            Box::new(|| println!("hi"))
        }
        fn returns_long_type_2() -> Thunk {
            Box::new(|| println!("hi"))
        }

        // Type Alias with **Parmeter**
        {
            /// Example type alias that takes a paremeter;
            /// reducing `Result<...>`boilerplate
            type Result<T> = std::result::Result<T, std::io::Error>;
            pub trait Write {
                fn write(&mut self, buf: &[u8]) -> Result<usize>;
                fn flush(&mut self) -> Result<()>;

                fn write_all(&mut self, buf: &[u8]) -> Result<()>;
                fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> Result<()>;
            }
        }
    }
    println!("---------------------------------------------\n");

    // `!`: the "never" or "bottom" type
    {
        /// our code signature indicates that this function will not return
        /// (diverges)
        fn nada_never() -> ! {
            unimplemented!()
        }

        /// again, can't return, just panics
        /// (diverges)
        fn disco_panic() -> ! {
            panic!("disco panic!");
        }

        /// internal use of `!` (bottom type)
        /// means that diverging arm (which returns `!`) can be coerced into
        /// `u32` (which is the return type of the function)
        /// Note that match must return a *single* type
        /// yet it returns <u32|!>
        /// the `!` is "coerced" into `u32`
        /// NOTE: the "coercion" phrasing is classic bad math verbalization
        ///       the point is that the type system is always consistent
        ///       and the `!` never_(inconsistent) type can be thought treat
        ///       as a **proof implementation detail** being coerced to i32
        ///       but it is just as valid, and clearer of meaning, to say that the `!`
        ///       is just not inconsistent with any "type-handoff"
        ///       (there is then some real discussion about what it means and why
        ///       -- e.g. you could make a type system with concrete instantiation
        ///       where there was coersion to specific concrete types or an overlapping
        ///       element .. or other things I'm sure [though I think this may indicate
        ///       issues for some type systems])
        ///       But the point we care about is (1) no inconsistency with `!` type receipt
        ///       and (2) that is achieved in these programs by diverging -- preventing hand-off
        ///       all together
        fn guesser(writ_val: &str) -> u32 {
            // return numrc_val
            match writ_val.trim().parse::<u32>() {
                Ok(num) => num,
                Err(_) => panic!("no number parse found!"),
            }
        }

        use rand::Rng;
        /// another example of using `!` type, but that doesn't blow up the entire programs
        /// instead it ensures that code that expects a number will always get a number
        /// (or not run), but combines this more gracefully to yield a vec of numbers,
        /// that *may* be empty
        fn rand_vec(guesses: u32) -> Vec<f64> {
            // import random library

            // guess a floating number between 1 and 100
            // let mut rng = rand::thread_rng();
            let mut vec = Vec::new();

            for _ in 0..guesses {
                let possible_val = rand::random::<f64>();
                // println!("possible_val: {}", possible_val);
                let def_val = match possible_val {
                    possible_val if possible_val > 0.5 => possible_val,
                    _ => continue,
                };
                vec.push(def_val);
            }
            vec
        }

        let test_vec = rand_vec(10);
        println!("test_vec: {:?}", test_vec);
        println!("len of test_vec: {}", test_vec.len());

        /// also loops are candidates for the `!` syntax
        fn this_is_the_song_that_never_ends() -> ! {
            loop {
                println!("this is the song that never ends");
            }
        }

        // this_is_the_song_that_never_ends();
    }
    println!("---------------------------------------------\n");

    // `Sized` & `?Sized`
    {
        /// same as `generic_2`, but with the trait restriction (`T: Sized`) implicit
        fn generic_1<T>(t: T) -> T {
            t
        }

        /// same as `generic_1`, just explicit about the trait restriction
        fn generic_2<T: Sized>(t: T) -> T {
            t
        }

        // | `generic_1` === `generic_2`
        // , but:
        /// accepts non-sized generics, but NOTE: it takes a *reference* to the value
        /// not the value itself
        fn generic_different<T: ?Sized>(t: &T) -> &T {
            t
        }

        // /// This won't compile.
        // /// despite labelling T as `?Sized` a return value of known size is still
        // /// required
        // fn generic_different_no_compile<T: ?Sized>(t: &T) -> T {
        //     *t
        // }
    }
    println!("---------------------------------------------\n");
}
