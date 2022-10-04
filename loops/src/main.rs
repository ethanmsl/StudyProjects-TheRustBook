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
}
