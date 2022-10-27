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

///////////////////////////////////////////////////////////////////////////////
