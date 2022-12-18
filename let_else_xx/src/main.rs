//! just playing with syntax, particularly `let-else` and variants
use log;

fn main() {
    // write a let statement declaring a result type
    println!("---------------------------------\n");

    let opt: Option<i32> = Some(111);
    dbg!(opt);

    let Some(x) = opt else {
        println!("res is None");
                    println!("returning...");
return;
    };
    dbg!(x);
    println!("---------------------------------\n");

    // let opt: Option<i32> = None;
    dbg!(opt);

    let Some(x) = opt else {
println!("\nno value\n");
println!("value of 'opt' is {:?}", opt);
                        println!("returning...");
        return;
    };
    dbg!(x);

    println!("---------------------------------\n");

    let res: Result<i32, &str> = Ok(222);
    dbg!(res);

    let Ok(x) = res else {
        println!("res is error {}", res.unwrap());
        println!("returning...");
        return;
    };
    dbg!(x);
    println!("---------------------------------\n");

    // let res: Result<i32, &str> = Err("error");
    dbg!(res);

    let Ok(x) = res else {
        println!("res is error {:?}", res);
        println!("returning...");
        return;
    };
    dbg!(x);
    println!("---------------------------------\n");

    // let res: Result<i32, &str> = Err("error");
    dbg!(res);

    let Ok(x) = res else {
        let err = res.unwrap_err();
        println!("this should print: {}", err);
        println!("returning...");
        return;
    };
    dbg!(x);
    println!("---------------------------------\n");

    // let res: Result<i32, &str> = Err("error");
    dbg!(res);

    let Ok(x) = res else {
        res.expect("\n--printed from expect method--\n");
        println!("this shouldn't print");
        println!("returning...");
        return;
    };
    dbg!(x);
    println!("---------------------------------\n");

    let res: Result<i32, &str> = Err("error_for_you");
    dbg!(res);

    let Ok(val) = res else {
        let err = res.unwrap_err();
        eprintln!("Failed to get value: {err}");
        return;
    };
}
