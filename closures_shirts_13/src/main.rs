#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
        //                            ^ crux of the logic
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

use std::thread;

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // // typed closure
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // // some examples of a function's vs various closures' syntaces
    // // NOTE: v3 & v4 would make type inferences from the type supplied as an arg
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

    // // NOTE: the first use of the closure will type it (by inference), potentially
    // // preventing the second, otherwise valid, use from working due to type-mismatch
    // let example_closure_01 = |x| x;
    // let s = example_closure_01(String::from("hello"));
    // let n = example_closure_01(5);
    // let example_closure_02 = |x| x;
    // let n = example_closure_02(5);
    // let s = example_closure_02(String::from("hello"));

    println!();

    // Demonstrating that the closure, in this case, chooses an immutable ref
    let list = vec![1,2,3];
    println!("Before defining closure:  {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    println!();

    // Demonstrating that the closure, in this case, chooses an mutable ref
    let mut list = vec![1,2,3];
    println!("Before defining closure:  {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    println!();

    // forcing closure to take ownership before shiping to new thread
    let list = vec![1,2,3];
    println!("Before defining closure:  {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    // println!("After calling closure: {:?}", list);  //<-- error
}
