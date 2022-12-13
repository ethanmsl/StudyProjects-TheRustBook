//! similar blog behavior, but written in a nominally more rust idiomatic way

/// Posted-Post
/// labelled `Post_r` to avoid name collision with `blog_oop_trad::Post`
pub struct Post_r {
    content: String,
}

/// Pre-Review Post
pub struct DraftPost {
    content: String,
}

impl Post_r {
    /// Builder -- creates a draft-post
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    /// get content from Post
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    /// add text to a draft post
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
