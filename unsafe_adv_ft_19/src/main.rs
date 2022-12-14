//! Unsafe Rust -- Ch 19  Advanced Features

fn main() {
    println!("----------------------------------------\n");

    // Creating Raw Pointers and Safe & Unsafe use
    {
        let mut num = 5;

        // casting *ref* to `num` as raw pointer*s*
        // NOTE: no `unsafe` keyword needed to *create* the raw pointers
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        println!("safe: r1 is: {:?}", r1);
        println!("safe: r2 is: {:?}", r2);
        unsafe {
            println!("unsafe: r1 is: {:?}", r1);
            println!("unsafe: r2 is: {:?}", r2);
            println!("unsafe: *r1 is: {:?}", *r1);
            println!("unsafe: *r2 is: {:?}", *r2);
        }

        let address = 0x012345usize;
        let r = address as *const i32;

        println!("safe: r is: {:?}", r);
        unsafe {
            // println!("unsafe: *r is: {:?}", *r);
            // // ^ lol, resulted in `segmentation fault`
        }
    }
    println!("----------------------------------------\n");

    // Unsafe functions and methods
    {
        unsafe fn dangerous() {
            println!("I bet you didn't know that I was dan-ger-ous\nI bet you didn't know someone could love you this much.\n-Big Data");
        }

        // dangerous();
        // // ^ can't be called outside of an `unsafe` block
        unsafe {
            dangerous();
        }

        // example of existing unsafe function safely wrapped
        // `.split_at_mut(_)`
        {
            let mut v = vec![1, 2, 3, 4, 5, 6];
            println!("v: {:?}, len(v): {}", v, v.len());
            let r = &mut v[..];
            let (a, b) = r.split_at_mut(3);
            //                          ^ specifies place *between* elements
            //                            re: 0-indexing meaning
            assert_eq!(a, &mut [1, 2, 3]);
            assert_eq!(b, &mut [4, 5, 6]);
        }

        // seeing if our `split_at_mut_bespoke` works
        // huh ... it does
        // so this "safe abstraction" is just ... not explicitely
        // labelling a function as unsafe and instead putting an unsafe block
        // inside it...  ...'k
        {
            do_a_thing();
            let mut v = vec![1, 2, 3, 4, 5, 6];
            println!("v: {:?}, len(v): {}", v, v.len());
            let r = &mut v[..];
            let (a, b) = split_at_mut_bespoke(r, 3);
            //                          ^ specifies place *between* elements
            //                            re: 0-indexing meaning
            assert_eq!(a, &mut [1, 2, 3]);
            assert_eq!(b, &mut [4, 5, 6]);
            i_did_a_thing();
        }
    }
    println!("----------------------------------------\n");

    // `extern` for foreign function interface-ing
    {
        // taking from elsewhere and bringing here

        // `extern` keyword is used to specify that the function is implemented
        // WOW! -- I'm impressed/surprised that that just worked.
        // ... I suppose C is so integral to most computers that a lot of common
        // functions/libraries are easy to find ... ?
        extern "C" {
            fn abs(input: i32) -> i32;
        }

        println!("Abs val C: {}", unsafe { abs(-3) });
        //                          ^ calling unsafe here!
        //              not sure if best practice to not frontload the `unsafe` keyword
        //              (from a visibility & clarity perspective)
        unsafe {
            //  ^ or putting whole term in block
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }

        // setting up to send from here to elsewhere
        #[no_mangle]
        pub extern "C" fn call_from_c() {
            println!("Just called a Rust function from C!");
        }
        // NOTE: ^ the above is for compiling into a *libary* and then linking in
        //         another program
        //         I don't think it would make sense defined in `main.rs`, but
        //         still, it's an example of syntax
        //         also NOTE: this use of extern, naturally, doesn't require "`unsafe`"
    }
    println!("----------------------------------------\n");

    // modifying `static` variables
    // (`static` variables are the same as `const` variables, including use of
    //  "SCREAMING_SNAKE_CASE" norms.
    //  , but `static` variables have a **fixed memory location** (at least relative
    //  to whatever hardware abstractions rust gets access to... (??))
    //  and *can* be mutable, but `static mut` can only be used (including read)
    //  via `unsafe` methods
    {
        static HELLO_WORLD_s: &str = "Hello, world!";
        const HELLO_WORLD_c: &str = "Hello, world!";
        println!("static 'HELLO_WORLD': {}", HELLO_WORLD_s);
        println!("const  'HELLO_WORLD': {}", HELLO_WORLD_c);

        static mut COUNTER: u32 = 0;

        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
        
        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }

        add_to_count(3);
        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }
    println!("----------------------------------------\n");
}

fn do_a_thing() -> () {
    println!("I'm doing a thing!");
}
fn i_did_a_thing() -> () {
    println!("I did a thing!");
}

// remake of `split_at_mut`
use std::slice;

fn split_at_mut_bespoke(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
