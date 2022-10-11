use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");
    // ^ diagnostic print

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "quit" {
            println!("Game Ended.");
            break;
        }
        // QUESTION: Why does guesss:String scope past loop?

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // ^ alternate way of dealing with the (ok|Err) enum from .parse()
        let guess: u32 = match guess.trim().parse() {
        //   ^ co-exists with guess:String
        //   QUESTION: it seems to be the same scope as guess:String
        //             does it count as shadowing if different type?
        //             If it were mutable could you not (as a matter of syntax)
        //             shadow the same type?
        //             (Can you shadow, immutable, the same type?)
        //  QUESTION: am I confusing shadowing (one synatix name eclipsing
        //            another for a period of time)
        //            with syntactic name implicitely holding its type
        //            and therefore existing as a separate variable?
            Ok(num) => num,
            Err(_) => {
                println!("I was not able to cast that input as a number.");
                println!("Please input a numeric value or 'quit'.");
                continue;
            },
        };

        println!("You guess-ed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    /*println!("-------------");
    let a = "babba";
    let b = "loo";
    println!("First var: {a}, second var: {b}");
    println!("First var: {}, second var: {}", a, b);
    */// ^5 checking different ways of embedding variable values in strings
}
