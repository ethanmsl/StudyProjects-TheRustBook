use closure_and_function_explorer_xx::add;

fn main() {
    println!("----------------------------------------\n");
    {
        println!("Hello, world!");
        println!("1 + 2 = {}", add(1, 2));
    }
    println!("----------------------------------------\n");

    let out_1 = add(1, 2);
    let out_2 = add(3, 4);
    {
        let cls_1 = || out_1 + out_2;
        println!("out_1 = {}", out_1);
        println!("out_2 = {}", out_2);
        println!("cls_1() = {}", cls_1());
    }
    println!("----------------------------------------\n");

    let tup = (24, 42);
    {
        let cls_2 = |x| x + tup.0 + tup.1;
        println!("tup.0 = {}", tup.0);
        println!("tup.1 = {}", tup.1);
        println!("cls_2(100) = {}", cls_2(100));
    }
    println!("----------------------------------------\n");

    // fn use_scope(x: i32) -> i32 {
    //     let out = x + tup.0 + tup.1;
    //     out
    // }
    // // ^ won't work -- our function can't use elements defined in it's scope
    // // capturing those values is for closures

    // {
    //     let cls_3 = |x|  tup.x;
    //     // ^ doesn't work becuase of syntax
    // }
    // println!("----------------------------------------\n");

    let arr = [1, 2, 3, 4, 5];
    {
        let cls_4 = |x| arr[x];
        println!("arr = {:?}", arr);
        println!("cls_4(0) = {}", cls_4(0));
        println!("cls_4(4) = {}", cls_4(4));
        // println!("cls_4(5) = {}", cls_4(5));
        // // ^ will panic in release and debug due to out of bounds
    }
    println!("----------------------------------------\n");

    let mut vec = vec![1, 2, 3, 4, 5];
    vec[0] += 10; // Fine, before closure even touches it
    {
        let cls_5 = |x| vec[x];
        // vec[0] += 100;  // PROBLEM: changes vec after touched, but before last look
        println!("vec = {:?}", vec);
        println!("cls_5(0) = {}", cls_5(0));
        println!("cls_5(4) = {}", cls_5(4));
        // println!("cls_5(5) = {}", cls_5(5));
        // // ^ will panic in release and debug due to out of bounds
        vec[0] += 1000; // Fine, after closure last looked at it
        println!("vec = {:?}", vec);
    }
    println!("----------------------------------------\n");

    let v_1 = vec![1, 2, 3, 5, 6, 7, 8, 9, 10];
    {
        let cls_2 = || &v_1[out_1..out_2];
        println!("v_1 = {:?}", v_1);
        println!("cls_2() = {:?}", cls_2());
    }
    println!("----------------------------------------\n");
}
