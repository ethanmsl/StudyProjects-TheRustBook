fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let shadows = 5;
    let shadows = shadows + 1;
    {
        let shadows = shadows * 2;
        println!("The value of shadows in the inner scope is: {shadows}");
    }
    println!("The value of shadows in the outer scope is: {shadows}");

    let arr: [isize; 3] = [-191; 3];
    println!("The 1st value of arr is: {}", arr[0]);
    println!("The 2nd value of arr is: {}", arr[1]);
    println!("The 3rd value of arr is: {}", arr[2]);
}
