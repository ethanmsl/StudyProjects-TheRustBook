// context:  there are two main IP address standards
enum IpAddrKind {
    V4,
    V6,
}

// Struct with an enum valued field
struct IpAddr_struct {
    kind: IpAddrKind,
    address: String,
}

// effectively a struct x enum with convenient syntax
// (will be improved below)
enum IpAddr_string {
    V4(String),
    V6(String),
}

// an improved improved mix of struct and enum
// like a hetero-product -- each enum value combining with a different struct
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

//---------------------------
// nice and useful example of an enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // stuff
    }
}

// Option type, as an example
enum Option<T> {
    None,
    Some(T)
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // route takes either enum of 'IpAddrKind'
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    //route(IpAddrKind);
    // ^ but this generates an error
    //   even though route takes 'IpAddrKind', it, speifically, seems to
    //   take an enum value, not the whole thing

    // Clunky Non-Exemplar
    // two instances of 'IpAddr' struct
    let _home_structured = IpAddr_struct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let _loopback_structured = IpAddr_struct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // nicer example
    // using an enum with... subtypes(?)
    let home = IpAddr_string::V4(String::from("127.0.0.1"));
    let loopback = IpAddr_string::V6(String::from("::1"));
    //
    // still nicer example
    let home2 = IpAddr::V4(127, 0, 0, 1);
}

fn route(IpAddrKind: IpAddrKind) {}
