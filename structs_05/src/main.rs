// Unit-Like Struct
struct AlwaysEqual;

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// defining struct
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct PointB {
    x: i32,
    y: i32,
}

impl PointB {
    fn get_x(&mut self) -> &mut i32 {
        &mut self.x
    }
}
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

    let mut user2 =
        build_user("user2@example.com".to_string(),
                   "name2".to_string());
    // NOTE: ^ the function doesn't build a mutable struct
    //         mutability is a feature of the variable
    //         it's how the compiler requires the handle to be used

    println!("user2's email, before change: {}", user2.email);
    user2.email = String::from("bbooppbboopp@example.com");
    println!("user2's email, after change: {}", user2.email);


    // Struct Update Syntax
    let _user3 = User{
        email: String::from("3rd@example.com"),
        //username: String::from("U3"),
        ..user1
        // ^ NOTE: this is nice
        // WARNING: this *MOVES* user1 values to user3
        //          WHERE MOVES CAN OCCUR
        //          (i.e. in 'username')
        //          and thus user1 no longer has ownership of its data
        //          (what becomes of the fields that aren't pointed to
        //          I'm not sure)
        //          Had neither username nor email been moved
        //          then all other values (stack values; implm copy trait) 
        //          would have been copied
        //          and user1 would not be moved.... (!!!)
    };

    // ------tuple structs
    let _black = Color(0,0,0);
    let _origin = Point(0,0,0);

    let _subject = AlwaysEqual;
    //---------------------------------

    /*
    let mut p = PointB {x:1, y:2};
    let x = p.get_x();
    *x += 1;
    println!("{} {}", *x, p.y);
    //                 ^   ^  both are refs to p, and x is a mutable ref
    //                 what the *x += 1 does is not yet clear to me...
    */ // will not compile


    let _abla = PointB {x:10, y:12};
    let mut babla = PointB {x:33, y:55};
    let bab = &babla.x;
    let bab2 = *bab + 2;
    println!("bab={bab}, bab2={bab2}");

    let mut p = PointB {x:1, y:2};
    let x = p.get_x();
    *x += 1;
    println!("x = {}", x);
    println!("*x = {}", *x);
}




// NOTE: special shorthand syntax for sending parameters to fields
//       (where both have same name)
fn build_user(email: String, username_param: String) -> User {
    User {
        email,  // Note this is shorthand for "email: email"
        username: username_param,  // this also works
        active: true,
        sign_in_count: 1,
    }
}
