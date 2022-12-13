//! implementing the '**state pattern**' in a "more traditional" OOP style

/// Primary struct of this module  
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

/// IMPL IMPL IMPL IMPL  
/// does this publish?
impl Post {
    /// Post constructor  
    /// (lol at phrase :)
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    /// add text to content field
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// get content from Post
    pub fn content(&self) -> &str {
        // adding this as a variable to get type inlining
        let boop = self.state.as_ref().unwrap();
        boop.content(self)
    }

    /// request review of Post
    pub fn request_review(&mut self) {
        // `.take()` to set state to none
        // ... to prevent use ... between now and then (not really clear)
        // ... Oh,  apparently this lets us take ownership of the value
        // ... though ... can we not own a structs value otherwise?
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    /// approve review of Post
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

/// state transitions defined on `State`s
trait State {
    // changes state from one thing to another that implements the State trait

    fn approve(self: Box<Self>) -> Box<dyn State>;
    // default implementation is to return empty-&str
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    /// changes state from `Draft` ~~> `PendingReview`
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    /// identity -- approval of 'Draft' should not be possible...
    /// (I think it should issue some sort of warning)
    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("Drafts cannot be approved without requesting review first");
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    /// identity function (doesn't actually change state)
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    /// approval: `PendingReview` ~~> `Published`
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    /// request_review on published content -- no erffect
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    /// approve on published content -- no effect
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    /// overrides default trait implementation
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

