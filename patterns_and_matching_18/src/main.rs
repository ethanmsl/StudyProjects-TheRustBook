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
        //

        // will warn as the conditional test will always be true
        if let y = 12 {
            println!("This should always print, if we reached the connected \"test\"")
        }
    }
    println!("----------------------------------------\n");

    // scope and shadowing
    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {:?}", x, y);
    }
    println!("----------------------------------------\n");

    // various syntaxes
    {
        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }

        let x = 5;
        // Only allowed for `char` and numeric types!!!
        // (apparently because those are the only types the compiler
        //  can check the range for emptyness)
        //  Very strange seeming to me .. I'd have thought ... hmm
        //  ... I was going to say anything with absolute ordering, but
        //  absolute ordering is not the same as finite-enumerability
        //  interesting perhaps
        //
        //  I suppose one could create an onto mapping from some set to numerics
        //  or chars
        match x {
            1..=5 => println!("one through five"),
            _ => println!("something other than [1, 5]"),
        }

        let x = 'c';
        // note that the `alpha..=beta` syntax seems to be much preferred here
        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }
    println!("----------------------------------------\n");
    // destructuring structs
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7 };
        // longhand form
        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        let p = Point { x: 1, y: 8 };
        // shorthand form
        let Point { x, y } = p;
        assert_eq!(1, x);
        assert_eq!(8, y);

        let p = Point { x: 3, y: 9 };
        // partial extraction shorthand form
        let Point { x, .. } = p;
        assert_eq!(3, x);

        let p = Point { x: 4, y: 5 };
        // partial extraction shorthand form
        if let Point { x: 4, y } = p {
            assert_eq!(5, y);
        } else {
            assert!(false);
        }

        let p = Point { x: 2, y: 0 };
        // partial extraction in a match statement
        match p {
            Point { x: 0, y } => println!("x is zero & y: {}", y),
            Point { x, y: 0 } => println!("y is zero & x: {}", x),
            Point { x, y } => println!("x: {}, y: {}", x, y),
        }

        // destructuring strucs and tuples
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
        println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);
    }
    println!("----------------------------------------\n");

    // destructuring enums
    {
        #[allow(dead_code)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y);
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
        }
    }
    println!("----------------------------------------\n");

    // destructuring nested structs and enums
    {
        #[allow(dead_code)]
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        #[allow(dead_code)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!(
                    "Change the color to hue {}, saturation {}, and value {}",
                    h, s, v
                )
            }
            _ => (),
        }
    }
    println!("----------------------------------------\n");

    // ignoring values in a pattern
    {
        /// This function takes two parameters, but only uses the second
        /// It uses a declaration nlike this `_: i32` and ignores the first
        /// , *but*, ~~I assume,~~ still type checks it -- which is kinda interesting
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }

        foo(3, 4);

        // foo('h', 5);
        // //   ^ yep, even though this value isn't used it's still type-checked

        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        println!("(pre-match) setting is {:?}", setting_value);

        match (setting_value, new_setting_value) {
            // matches if both values are of type `Som(T)`
            // *also* both types `T` must be the same
            // *not* because of the `Some(_), Some(_)` pattern
            // , but because one of the results in another branch involves
            // setting one to the value of the other and nothing is proteceting against
            // those mismatch variables getting there
            // (not sure if there is a nice way to do that ... perhaps the type algebra
            //  will allow it)
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }

        // let mut setting_value = Some('c');
        // let new_setting_value = Some(10);
        // match (setting_value, new_setting_value) {
        //     (Some(_), Some(_)) => {
        //         println!("Can't overwrite an existing customized value");
        //     }
        //     (Some(T), Some(X)) => {
        //         // interesting (1): this recognizes that `T` and `X` are types
        //         // and that it's getting a `char` and an `i32` fed to it, resp.
        //         println!("different some types");
        //     }
        //     (T, X) => {
        //         println!("different types");
        //     }
        //     _ => {
        //         setting_value = new_setting_value;
        //         // interesting (2): this does not recognize that non-equal types
        //         // won't be able to make it to this branch :|
        //     }
        // }
        ///////////////////////////////////////////////////////////////////////////
        //         // Copied for easier discovery:
        //         // interesting (2): this does not recognize that non-equal types
        //         // won't be able to make it to this branch :|

        println!("(post-match) setting is {:?}", setting_value);
    }
    println!("----------------------------------------\n");

    // more patter ignoring
    {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => println!("Some numbers: {first}, {third}, {fifth}"),
        }
    }
    println!("----------------------------------------\n");

    // binding vs non binding with `_*`
    {
        let s = Some(String::from("Hello!"));

        if let Some(_s) = s {
            println!("found a string");
        }

        // println!("{:?}", s);
        // // ^ will cause an error because `_s` above took ownership of the `s` value

        let s = Some(String::from("Hello!"));

        if let Some(_) = s {
            println!("found a string");
        }

        println!("{:?}", s);
        // ^ this is okay becaue the `_` didn't bind the value
    }
    println!("----------------------------------------\n");

    // ignoring parts with ..
    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }

        let numbers = (2, 4, 8, 16, 32);

        // this is an **un**ambiguous use of `..`
        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            }
        }

        // // but this is an AMBIGUOUS use of `..`
        // // (compiler error says that `..` can only be used once per tuple pattern)
        // match numbers {
        //     (.., second, ..) => {
        //         println!("A number: {}", second);
        //     }
        // }

        struct LotsOfNumbers {
            a: i32,
            b: i32,
            c: i32,
            d: i32,
            e: i32,
        }

        let lots_of_numbers = LotsOfNumbers {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: 5,
        };

        // // even though we're using shorthand naming conventions here (or writing in a
        // // way that would be consistent with it)
        // // the extra `..` is not allowed
        // // in fact the compiler error says that the `..` bust be
        // // at the end of the pattern
        // match lots_of_numbers  {
        //     LotsOfNumbers { .., b, .. } => println!(" b: {}", b),
        // }

        // // also not allowed
        // // the `..` must be at the **end** of the struct destructuring pattern
        // match origin {
        //     Point { x, z, .. } => println!("x is {} and z is {}", x, z),
        // }

        // this **IS** allowed, even though `x` and `z` are not 'contiguous'
        // in the struct declaration
        // (now that I think about it I'm not sure there is any real ordering there
        //  , in the object we create)
        match origin {
            Point { x, z, .. } => println!("x is {} and z is {}", x, z),
        }
    }
    println!("----------------------------------------\n");

    // match Guards!!! :)
    {
        let num_vec = vec![Some(4), Some(5), Some(6), Some(7)];

        for num in num_vec {
            // Neat and nice! :)
            match num {
                Some(x) if x % 2 == 0 => println!("The number {} is even", x),
                Some(x) => println!("The number {} is odd", x),
                None => (),
            }
        }

        // // this doesn't work because num_vec was moved in the above case!
        // // (and also used had an `into_iter` call on it)
        // for num in num_vec.iter() {
        //     // Neat and nice! :)
        //     match num {
        //         Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        //         Some(x) => println!("The number {} is odd", x),
        //         None => (),
        //     }
        // }

        let x = Some(5);
        let y = 10;

        // with match guard
        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {}", n),
            _ => println!("Default case, x = {:?}", x),
        }

        // without match guards
        match x {
            Some(50) => println!("Got 50"),
            Some(n) => {
                if n == y {
                    println!("Matched, n = {}", n)
                }
            }
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {}", x, y);
    }
    println!("----------------------------------------\n");

    // precedence clarification wrt match guards
    {
        let x = 4;
        let y = false;

        // note that the `if y` applies to all the or'd conditions (`4 | 5 | 6`)
        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }

        // rust will complain about unecessary parentheses
        // , but I'm inclined to think that's still the better choice
        #[allow(unused_parens)] // will only avoid for some def of locally - great
        // (see below example that still gets a warning)
        match x {
            (4 | 5 | 6) if y => println!("yes"),
            _ => println!("no"),
        }

        // rust will complain about unecessary parentheses
        // , but I'm inclined to think that's still the better choice
        match x {
            (4 | 5 | 6) if y => println!("yes"),
            _ => println!("no"),
        }
    }
    println!("----------------------------------------\n");

    // `@` bindings
    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 18 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range [3, 7]: {}", id_variable),

            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range ( [10,12] )")
            }

            Message::Hello { id: boops } if boops == 13 => {
                println!("Found an id equal to thirteen: {}", boops)
            }

            // if id is in range 15..=20
            Message::Hello { id } if (14..=20).contains(&id) => {
                println!("Found an id inclusively betwen fourteen and fifteen: {}", id)
            }

            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
    println!("----------------------------------------\n");
}
