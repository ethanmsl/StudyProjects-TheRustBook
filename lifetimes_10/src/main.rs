#![allow(dead_code)]
// struct with reference fields requires lifetimes
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael.  Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    //                                     ^ first call goes to first element
    println!("{}", first_sentence);
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let s: &'static str = "I have a static lifetime.";

    // // won't run as x does not live long enough to be used by r later
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);

    let string1 = String::from("abcd");
    let result1;
    // let result2;
    {
        // works ... for some reason ... string2 as str has a long lifetime?
        let string2 = "xyz";
        result1 = longest(string1.as_str(), string2);

        // // doess not work
        // let string3 = String::from("xyz");
        // result2 = longest(string1.as_str(), string3.as_str());
    }
    println!("the longest string is {}", result1);
    // println!("the longest string is {}", result2);
}

// // as written failes because it requires a 'lifetime specifier'
// // this makes some sense in the context of our outputs:
// // either x or y, which could have different scopes/lifetimes
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// since it returns something without using y doesn't need lifetime annotation
fn first_argument<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
//                                                             ^ QUESTION: why no lifetime?
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
