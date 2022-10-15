// #[allow(dead_code)]

#[allow(unused_variables)]
fn main() {
    // // this will result in a compiler error; it needs a type declaration
    // let mut v_mysterious_type = Vec::new();

    let v_tbd: Vec<i32> = Vec::new();
    let v_known = vec![1, 2, 3];

    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    let v_boop = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v_boop[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v_boop.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let twentieth: Option<&i32> = v_boop.get(20);
    match twentieth {
        Some(twentieth) => println!("The twentieth element is {}", twentieth),
        None => println!("There is no twentieth element."),
    }

    // auto-looping through a vector
    let v = vec!['a', 'b', 'c'];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![0, 1, 2];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{i}");
    }


// -----------------------------------
    // enum typed vectors
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for row in &row {
        println!("This row from row is {:?}", row);
    }


}
