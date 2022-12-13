//! Ch 18 -- on patterns and matching
#![allow(unused_variables)]

fn main() {
    println!("----------------------------------------\n");

    // mixing `if let`, `else if let`, `else if`, and `else`
    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your faovrite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }
    println!("----------------------------------------\n");

    // `while let` conditional loop
    {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        println!("stack vector: {:?}", stack);
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }
    println!("----------------------------------------\n");

    // `for loop` 'destructuring' pattern
    {
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }
    println!("----------------------------------------\n");

    // `let` statements:  `let PATTERN = EXPRESSION`
    {
        let (x, y, z) = (1, 2, 3);
        // let (w, r) = (1, 2, 3);
        // //         ^ produces a compile time error


        // give me a result type with a specific error type

        let boop = "42".parse::<u64>();
        let bop = Ok::<u64, std::num::ParseIntError>(42);
        let beep = Err::<u64, _>("boop");
        println!("boop: {:?}, beep: {:?}", boop, beep);
        // let Ok(bopped) = bop;
        // // ^ "refutable pattern"; not accepted
        // println!("pattern unmatched bop: {}", bopped);
    }
    println!("----------------------------------------\n");

    // function parameters as patterns
    {
        /// destructures a tuple input
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: (-{}= ={}-)", x, y);
        }


        let point = (3, 5);
        print_coordinates(&point);
    }
    println!("----------------------------------------\n");
{
        let maybe_num: Option<u64> = "12".parse().ok();
        //                           ^ convert Result~~>Option
        if let Some(x) = maybe_num {
            // x is (appropriately) confined to this scope! :)
        }

        // println!("x : {:?}", x);
        // // ^ because x is confined to the above (conditional) scope it is not 
        // //   available here
    }
    println!("----------------------------------------\n");
}
