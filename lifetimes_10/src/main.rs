fn main() {
    // // won't run as x does not live long enough to be used by r later
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);

    let string1 = String::from("abcd");
    let result1;
    // let result2;
    {
        // works ... for some reason ... string2 as str has a long lifetime?
        let string2 = "xyz";
        result1 = longest(string1.as_str(), string2);

        // // doess not work
        // let string3 = String::from("xyz");
        // result2 = longest(string1.as_str(), string3.as_str());
    }
    println!("the longest string is {}", result1);
    // println!("the longest string is {}", result2);
}

// // as written failes because it requires a 'lifetime specifier'
// // this makes some sense in the context of our outputs:
// // either x or y, which could have different scopes/lifetimes
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// since it returns something without using y doesn't need lifetime annotation
fn first_argument<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
