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

// generic struct -- single type variable
#[derive(Debug)]
struct Point_homo<T> {
    x: T,
    y: T,
}

// generic struct -- two types variables
// NOTE: Point<T> & Point<T,U> are not allowed to both exist
//       Overloading/~multiple-dispatch is not enabled for these structs
//       (probably for the best -- but I can see arguments either way)
#[derive(Debug)]
struct Point_hetero<T, U> {
    x: T,
    y: U,
}

// implementing typed methods for Point_homo(<T>)
impl<T> Point_homo<T> {
    fn x_val(&self) -> &T {
        &self.x
    }

    // // this is not allowable if we have the same-named function in the 
    // // type-specific impl(ementation) block below
    // // (likely TLDR: there's no allowed multiple-dispatch/function overloading
    // //               allowed.  -- if we wanted that sort of functionality
    // //               we'd, presumably, do it with internal logic in a function
    // //               accepting generic types)
    // fn distance_from_origin(&self) -> f32 {
    //     return 0.0
    // }
}

//NOTE: we can implement methods for specific types only!
//QUESTION: what's the story on generic and specific methods with name collision
//         hierarchy of defaults or unallowable conflict?
// here we define methods for Point_homo specific to f32 float type
impl Point_homo<f32>{
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
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
    let integer = Point_homo {x: 5, y: 10 };
    let float = Point_homo { x: 1.0, y: 4.0 };
    let float2: Point_homo<f32> = Point_homo { x: 1.0, y: 4.0 };
    println!("struct 'integer': {:?}", integer);
    println!("struct 'float': {:?}", float);
    println!("struct 'float2': {:?}", float2);
    // // Point_homo<T> operates over a single type
    // // notable, 5 & 4.0 aren't both cast/interepreted as floats, instead it
    // // refuses to compile -- (this desire for clarity, 'to be clear what you 
    // // meant was...' seems like quite a good thing)
    // let wont_work = Point_homo { x:5, y: 4.0 };
    // but we can define a Point_hetero<T,U> that allows different typed fields
    let will_work = Point_hetero { x:5, y: 4.0 };
    println!("struct 'will_work': {:?}", will_work);
//---------------------------------------------------
    // using typed methods
    let p = Point_homo { x: 5, y: 10 };
    println!("p.x = {}", p.x_val());

    // we can define methods that only operate on specific types
    let p_f32: Point_homo<f32> = Point_homo { x: 12.0, y: 11.11 };
    let dist = p_f32.distance_from_origin();
    println!("point at: {:?}, which has dist from origin: {},", p_f32, dist);
}


