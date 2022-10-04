fn main() {
    /*
     * Not much to see here; pretty standard conditionals
     * EXCEPT the last section
     */
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // ^ conditionin assignment

    println!("The value of number is: {number}");

    // prints "()" with debug formatting
    // ':?' is a formatting option
    //      triggers use of 'std::fmt::Debug'
    let x = true;
    let y = if x {};
    println!("Debug-formatted value of y: {y:?}");
}
