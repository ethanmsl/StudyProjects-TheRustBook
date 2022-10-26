#![allow(unused_variables)]

use aggregator::{Summary, Tweet};
mod aggregator;  // <-- sufficient along with raw path use for Tweet and say_hi()
                 //     , but not sure hot to call the Summary trait with raw path

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let boop = 12 *3;

    println!("1 new tweet: {}", tweet.summarize());
    tweet.say_hi();
}
