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
}

// remake of `split_at_mut`
use std::slice;

fn do_a_thing() -> () {
    println!("I'm doing a thing!");
}
fn i_did_a_thing() -> () {
    println!("I did a thing!");
}

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
