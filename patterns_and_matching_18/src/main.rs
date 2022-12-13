//! Ch 18 -- on patterns and matching

fn main() {
    println!("----------------------------------------\n");

    // mixing `if let`, `else if let`, `else if`, and `else`
    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your faovrite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }
    println!("----------------------------------------\n");

    // `while let` conditional loop
    {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        println!("stack vector: {:?}", stack);
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }
    println!("----------------------------------------\n");

    // `for loop` 'destructuring' pattern
    {
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }
    println!("----------------------------------------\n");
}
