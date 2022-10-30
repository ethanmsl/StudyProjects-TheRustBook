////////////////////////// integration_tests.rs //////////////////////////
use test_organization_11;

// NOTE: we can run a specific test file with `cargo test --test filename`
//       (no `.rs` needed for the filename)

#[test]
fn it_adds_two() {
    assert_eq!(4, test_organization_11::add_two(2));
}


