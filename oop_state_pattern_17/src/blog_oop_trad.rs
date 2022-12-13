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
}

trait State {}

struct Draft {}

impl State for Draft {}
