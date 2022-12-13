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
        ""
    }

    /// request review of Post
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    /// changes state from one thing to another that implements the State trait
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    /// changes state from `Draft` ~~> `PendingReview`
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    /// identity function (doesn't actually change state)
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
