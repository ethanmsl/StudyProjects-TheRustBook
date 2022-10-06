fn main() {
    // instantiating an instance of struct
    // NOTE: order of field definition doesn't matter
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // access fields with dot-notation
    println!("user1's email: {}", user1.email);

    // let mut user2 = user1.copy();
    // ^ didn't work '.copy()' method didn't work

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user2's email, before change: {}", user2.email);
    user2.email = String::from("bbooppbboopp@example.com");
    println!("user2's email, after change: {}", user2.email);

}


// defining struct
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
