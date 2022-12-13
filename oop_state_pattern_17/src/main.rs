//! main -- can be set to run from multiple implementations of core

use oop_state_pattern_17::blog_oop_trad::Post;
use oop_state_pattern_17::blog_rust_vrnt::{Post_r, DraftPost};

fn main() {
    // Uses `blog_oop_trad` module
    {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    // Uses `blog_rust_vrnt` module
    {
        let mut post = Post_r::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());
    }
}
