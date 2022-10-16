// i32 accepting size sorter
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// char accepting size sorter
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic struct -- single type variable
#[derive(Debug)]
struct PointHomo<T> {
    x: T,
    y: T,
}

// generic struct -- two types variables
// NOTE: Point<T> & Point<T,U> are not allowed to both exist
//       Overloading/~multiple-dispatch is not enabled for these structs
//       (probably for the best -- but I can see arguments either way)
#[derive(Debug)]
#[allow(dead_code)]
struct PointHetero<T, U> {
    x: T,
    y: U,
}

// implementing typed methods for Point_homo(<T>)
impl<T> PointHomo<T> {
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

// example to show that type naming convenctions can be varied from underlying
// struct -- e.g. for purpose of clarity here
// (actually the books example is unclear here: text weems to suggest that as
// the purpose, but then it renames the struct's type variables in the same
// example ... :shrug:)
// WARNING: This can take types (a,b), (c,d) and put out (a,d)
//          ... the signature actually makes this clear, so that should be fine
//          ('warning' probably not needed here), but it's still important
//          to note ... even though this is an impl on xxx(a,b) and it returns
//          an xxx(...) it returns an xxx(a, _), with a potentially different
//          type signature.
impl<X1, Y1> PointHetero<X1, Y1> {
    //    ^                   ^    QUESTION: why do I declare those twice?
    //                                       can they be different?
    //                                       probably if I only want to pass some
    //                                       of the points variables to the methods
    //                                       the impl<...> def coudl be different
    //                                       e.g. if I had an impl block that only
    //                                       operated on one field of a struct
    //                                       NOTE: unsuccesfull attempt to implement that
    //                                       below in commented out Point3DHetero impl code
    //                                       MAYBE: it's just for listing traits...
    //                                       SEE: added comments (& stackoverflow link)
    //                                            that seem to answer it, with
    //                                            TLDR: declare all unique vars
    //                                                  in struct<...> signature
    // takex x from main point and y from a secondary input point
    // NOTE: the declaration of Type-Var Names for mixup is somewhat confusing...
    //       we declared 'mixup<X2, Y2>', which are the names we use for the
    //       accessory input, but X1, Y1 are still available from the
    //       'impl<X1, Y1>' declaration.
    //       what if mixup() had 2 different accessory inputs with separate
    //       type variables allowed...?
    //       ^ SEE: 'triple_mixup()' below, which looks to addres that
    fn mixup<X2, Y2>(self, other: PointHetero<X2, Y2>) -> PointHetero<X1, Y2> {
        // hmmm... I wanted to create a new point
        // , but I don't think I can do that without implementing
        // 'copy' on the input struct elements
        // so just going to allow a move instead
        PointHetero {
            x: self.x,
            y: other.y,
        }
        // returns the above Point
    }
}

#[allow(dead_code)]
struct Point3DHetero<T, U, V> {
    x: T,
    y: U,
    z: V,
}

#[allow(dead_code)]
impl<T, U, V> Point3DHetero<T, U, V> {
    // mix three points together
    // Point3DHeteero, other1, other2 -- where all three take different types
    fn triple_mixup<X2, Y2, Z2, X3, Y3, Z3>(
        self,
        other1: Point3DHetero<X2, Y2, Z2>,
        other2: Point3DHetero<X3, Y3, Z3>,
    ) -> Point3DHetero<T, Y2, Z3> {
        Point3DHetero {
            x: self.x,
            y: other1.y,
            z: other2.z,
        }
    }
}

// // This does not compile
// // I'm not sure what the story on impl signature and struct signatures is yet
// NOTE: see this stack overflow on impl<> sig declarations:
// https://stackoverflow.com/questions/45473626/why-does-rust-require-generic-type-declarations-after-the-impl-keyword/45473717#45473717
//  TLDR: looks like the following 3 styles are viable:
//        (1) impl<T, U, V> Structo<T, U, V> { ... }         <--full freedom
//        (2) impl          Structo<i32, u32, char> { ... }  <--concretes
//        (3) impl<U, V>    Structo<V, U, V> { ... }         <--repetition constraint
//        TLDR: I *think*, you have to declare every unique variable type in 
//                         the struct signature
//                  so: impl<A, D> Structo<A, A, i32, D> { ... }
//                    would be a thing, for example
//                    More and future testing to confirm/expand
//
// #[allow(dead_code)]
// impl<T> Point3DHetero<T, U, V> {
//     fn x_val(&self) -> &T {
//         &self.x
//     }
// }

//NOTE: we can implement methods for specific types only!
//QUESTION: what's the story on generic and specific methods with name collision
//         hierarchy of defaults or unallowable conflict?
// here we define methods for Point_homo specific to f32 float type
impl PointHomo<f32> {
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
    }

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
    let integer = PointHomo { x: 5, y: 10 };
    let float = PointHomo { x: 1.0, y: 4.0 };
    let float2: PointHomo<f32> = PointHomo { x: 1.0, y: 4.0 };
    println!("struct 'integer': {:?}", integer);
    println!("struct 'float': {:?}", float);
    println!("struct 'float2': {:?}", float2);
    // // Point_homo<T> operates over a single type
    // // notable, 5 & 4.0 aren't both cast/interepreted as floats, instead it
    // // refuses to compile -- (this desire for clarity, 'to be clear what you
    // // meant was...' seems like quite a good thing)
    // let wont_work = Point_homo { x:5, y: 4.0 };
    // but we can define a Point_hetero<T,U> that allows different typed fields
    let will_work = PointHetero { x: 5, y: 4.0 };
    println!("struct 'will_work': {:?}", will_work);
    //---------------------------------------------------
    // using typed methods
    let p = PointHomo { x: 5, y: 10 };
    println!("p.x = {}", p.x_val());

    // we can define methods that only operate on specific types
    let p_f32: PointHomo<f32> = PointHomo { x: 12.0, y: 11.11 };
    let dist = p_f32.distance_from_origin();
    println!(
        "point at: {:?}, which has dist from origin: {},",
        p_f32, dist
    );
    //---------------------------------------------------
    // type mix-alation
    //  P<a,b>.f(<P<c,d>) -> P<a,d>
    let p1 = PointHetero { x: 5, y: 10.4 };
    let p2 = PointHetero { x: "Hello", y: 'c' };
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    let p3 = p1.mixup(p2);
    println!("p3: {:?}", p3);
    // note: we can also call the individual fields even though
    //       the struct isn't declared public -- I suppose it's
    //       de facto public within scope (?) <--TODO: clarify that
    println!("p3.x: {}, p3.y: {}", p3.x, p3.y)
}
