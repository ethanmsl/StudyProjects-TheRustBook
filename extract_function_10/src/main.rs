fn main() {
    // definse some numbers
    let number_list = vec![34, 50, 25, 100, 65];

    // create changing val and seed it with a start
    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number from {:?} is: {}", number_list, largest);
}
