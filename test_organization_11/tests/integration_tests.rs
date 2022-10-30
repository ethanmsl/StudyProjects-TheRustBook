////////////////////////// integration_tests.rs //////////////////////////
use test_organization_11;

// bringing our subdirectory (same level as this file) into scope
mod common;

// NOTE: we can run a specific test file with `cargo test --test filename`
//       (no `.rs` needed for the filename)

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_organization_11::add_two(2));
}


