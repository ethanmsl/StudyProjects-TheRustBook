// i32 accepting size sorter
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    };

    largest
}

// char accepting size sorter
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    };

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

// generic accepting size sorter
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    };

    largest
}

#[allow(unused_variables)]
fn main() {
    //using i32 type specific code
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number in {:?} is: {}", number_list, result);

    // using char type specific code
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char in {:?} is: {}", char_list, result);
//---------------------------------------------------
//  Using Generic Types
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number in {:?} is: {}", number_list, result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char in {:?} is: {}", char_list, result);
//---------------------------------------------------
    // Using Generic Struct to take variable types
    //     seems very neat -- curious about specifics!
    let integer = Point {x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let float2: Point<f32> = Point { x: 1.0, y: 4.0 };
    // // Point<T> operates over a single type
    // // notable, 5 & 4.0 aren't both cast/interepreted as floats, instead it
    // // refuses to compile -- (this desire for clarity, 'to be clear what you 
    // // meant was...' seems like quite a good thing)
    // let wont_work = Point { x:5, y: 4.0 };
}


