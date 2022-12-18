//! just playing with syntax, particularly `let-else` and variants

fn main() {
    // write a let statement declaring a result type

    let res: Option<i32> = Some(111);

    let Some(x) = res else {
    return;
    };

    dbg!(x);

    let res: Option<i32> = None;

    let Some(x) = res else {
    println!("\nno value\n");
    return;
    };
    dbg!(x);
}
