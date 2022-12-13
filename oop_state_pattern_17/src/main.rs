//! main -- can be set to run from multiple implementations of core


use oop_state_pattern_17::blog_oop_trad::Post;
// use blog_rust_varnt::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    // post.request_review();
    // assert_eq!("", post.content());
    //
    // post.approve();
    // assert_eq!("I ate a salad for lunch today", post.content());
}
