/////////////////////////////////////aggregator////////////////////////////////
// pub mod aggregator {
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
    pub fn say_hi(&self) {
        if self.reply {
            println!("reply was True")
        } else {
            println!("reply was False")
        }
    }
}
// }
///////////////////////////////////////////////////////////////////////////////
