#![allow(unused_variables)]

/////////////////////////////////////aggregator////////////////////////////////
mod aggregator {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // we list the trait that we're implementing
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    // we list the trait that we're implementing
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    impl Tweet {
        fn say_hi() {
            println!("Hi");
        }
    }
}
///////////////////////////////////////////////////////////////////////////////


// use aggregator::{Summary, Tweet};
// mod aggregator;

fn main() {
    let tweet = aggregator::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    // println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.(aggregator::say_hi()));
}
