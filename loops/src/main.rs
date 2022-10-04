fn main() {
    // counter with condition-gated break
    let mut loop_counter = 0;
    loop {
        println!("again!");
        loop_counter += 1;

        if loop_counter >= 5 {
            break;
        }
    }

    // counter with condition-gated break
    // that returns a value <-- NEAT
    let mut counter = 0;
    let result = /*NEAT*/ loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // *labelled* loops
    // with label-pointed break
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        // ^ Reset every up loop

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
                // Breaks out of non-immediate parent loop
                // by using loop-label to point break at it
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while loop syntax
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("countdown while loop exited");
    //println!("can I embed a number and have it print? zero: {0}");
    // ^ No, I could not. (got commentary on invalid ref to positional arg)

    // for loop syntax
    let arr = [10, 20, 30, 40, 50];
    // ^ appears that arr's type is compiler inferred
    // ('[i32;5]', with 'i32' breaking symmetry by way of preference)
    for element in arr {
        println!("the value: {element}");
    }

    // for loop countdown (range syntax)
    for number in 1..=3 {
        println!("for-loop countdown: {number}")
    }
    println!("for-loop countdown exited");

    println!("-------------------------");
    for number in 1..=10 {
        let fib_num = fibonacci_nth(number);
        let postfix = match number {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        println!("The {number}{postfix} Fibonacci number is {fib_num}");
    }
}


fn fibonacci_nth(n: u32) ->u32 {
    let mut prev = 0;
    let mut current = 1;
    let mut temp_value;

    for _ in 1..=n {
        // new val
        temp_value = prev + current;

        // shift values
        prev = current;
        current = temp_value;
    }
    current
}
