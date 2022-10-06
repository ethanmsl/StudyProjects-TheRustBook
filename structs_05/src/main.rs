fn main() {
    // instantiating an instance of struct
    // NOTE: order of field definition doesn't matter
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}


// defining struct
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
