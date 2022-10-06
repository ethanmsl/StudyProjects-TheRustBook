// context:  there are two main IP address standards
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

// Struct with an enum valued field
struct IpAddr_struct {
    kind: IpAddrKind,
    address: String,
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
    let home = IpAddr::V4(String::from("127.0.0.1"))
    let loopback = IpAddr::V6(String::from("::1"))
}

fn route(IpAddrKind: IpAddrKind) {}
