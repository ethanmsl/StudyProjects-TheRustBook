////////////////// common_unintended_inclusion_as_a_test.rs /////////////////
// Here to simulate a file of helper functions used by multiple test files
// EXCEPT we've mistakenly included it as a test file
// putting it in a subdirectory of `/tests/` will allow other to use it
// without it being compiled as a crate and run and included in our `cargo test` output

pub fn setup() {
    // setup code specific to your library's tests would go here
}
