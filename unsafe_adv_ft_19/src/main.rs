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
    }
}
