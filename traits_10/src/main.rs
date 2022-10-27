// #![allow(unused_variables)]

use aggregator::{Summary, Tweet};

use crate::aggregator::NewsArticle;
mod aggregator;  // <-- sufficient along with raw path use for Tweet and say_hi()
                 //     , but not sure hot to call the Summary trait with raw path

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let tweet2 = Tweet {
        username: String::from("bird_speaker"),
        content: String::from("caw-caw-chirp"),
        reply: true,
        retweet: true,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("----------------------------------");
    tweet.say_hi();
    println!("----------------------------------");


    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hocky team in the NHL.",
            ),
    };

    println!("New article available! {}", article.summarize());
    println!("----------------------------------");

    aggregator::notify(&tweet);
    aggregator::friends_view(&article);
    aggregator::to_compare(&tweet, &tweet2);
}
