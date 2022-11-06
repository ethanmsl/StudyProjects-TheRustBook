fn largest(list: &[i32]) -> &i32 {
    // create changing var and seed it with a starter val
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    };

    largest
}

fn main() {
    // definse some numbers
    let number_list = vec![34, 50, 25, 100, 65];
    let largest_result = largest(&number_list);
    println!("The largest number from {:?} is: {}", number_list, largest_result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // NOTE: if above 'largest_result' is simply 'largest' it conflicts
    //      with 'largest(...)' function -- indeed even rename lsp action
    //      treats the function name as such (though treesitter highlighting
    //      doesn't seem to have a problem...)
    let largest_result = largest(&number_list);
    println!("The largest number from {:?} is: {}", number_list, largest_result);
}
