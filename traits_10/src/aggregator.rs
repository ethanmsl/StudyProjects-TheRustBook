/////////////////////////////////////aggregator////////////////////////////////
// pub mod aggregator {
pub trait Summary {
    fn summarize(&self) -> String {
        // a default implementation
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// we list the trait that we're implementing
impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    //   ^ by having the block be blank the default implementation is used,
    //   and the Struct is earmarked to have the trait.
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
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

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Tweet {
    pub fn say_hi(&self) {
        if self.reply {
            println!("reply was True")
        } else {
            println!("reply was False")
        }
    }
}

// function that takes a type that implements summary.... (I suppose)
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// equivalent 'trait bound' syntax to the above:
pub fn friends_view<T: Summary>(item: &T) {
    println!("Your friends recently read: {}", item.summarize());
}

// 'trait bound' syntax in a case where it's easier to read
pub fn to_compare<T: Summary>(item1: &T, item2: &T) {
    //                        ^ note: those must both be of the same precise type
    //                               not merely any times implementing trait
    //                               (e.g. two tweets or two newsarticles
    //                               , but not one of each)
    println!(
        "Two items worth comparing are:\n---{}\n---{}",
        item1.summarize(),
        item2.summarize()
    );
}

// // two examples of '+' notation to specify multi trait boundedness
// pub fn notify(item: &(impl Summary + Display)) {...}
// pub fn notify<T: Summary + Display>(item: &T) {...}

// // 'where' clause syntax
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     unimplemented!()
// }

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("robomade"),
        content: String::from("krrrrchnk"),
        reply: true,
        retweet: true,
    }
}


// methods can be also be conditionally implemented, for structs with genetic types, for example
// e.g.:
use std::fmt::Display;

#[allow(dead_code)]
struct Pair<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[allow(dead_code)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


// // trait conditions can also be used on trait implementations
// // e.g.:
// impl<T: Display> ToString for T {
//     // --snip--
// }


///////////////////////////////////////////////////////////////////////////////
