#[derive(Debug)]
enum UsState {
    _Alabama,
    _Alaska,
    _California,
    Texas,
    _Vermont,
}
/// I'm writing documentation code.
/// Nominally this supports Markdown syntax.
/// *bold* and _italic_.
/// , but apparently that doesn't get picked and highlighted
/// by my lsp & TreeSitter interface
enum Coin {
    _Penny,
    Nickle,
    _Dime,
    Quarter(UsState),
    // ^ Quarter along with info about which state is on it
}

#[derive(Debug)]
enum Either {
    Left(usize),
    Right(String),
}

fn main() {
    // Coin stuff
    let nickle_coin = Coin::Nickle;
    println!("Value of a Nickle: {}", value_in_cents(nickle_coin));
    let texas_quarter = Coin::Quarter(UsState::Texas);
    println!("Value of a Quarter: {}", value_in_cents(texas_quarter));

    // Adding to 'maybe' stuff
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
//-----------------------------------------------

    let from0 = decr_twice_v1(0);
    println!("decr_twice_v1(0) = {:?}", from0);

    let from1 = decr_twice_v1(1);
    println!("decr_twice_v1(1) = {:?}", from1);

    let from2 = decr_twice_v1(2);
    println!("decr_twice_v1(2) = {:?}", from2);

//-----------------------------------------------
    let x = Either::Right(String::from("Hello world!"));
    let value = wonkadoo(&x);
    println!("wonkadoo(x): {value}, x: {x:?}");

}

fn wonkadoo(x: &Either) -> usize {
    match x {
        Either::Left(n) => *n,
        Either::Right(s) => s.len(),
    }
}

fn decr_twice_v1(n:u32) -> Option<u32> {
    match n {
        0 => None,
        1 => None,
        n2 => Some(n2 - 2),
        //     ^ I'm surprised this is needed, I would have thought that
        //     any type of Option enum would be valid
        //     why does 'None' work but 'Some(n2 - 2)' doesn't?
        //     ANSWER: because the types of Option<T> are None & Some<T>
        //             (NOT None & T)     :)
        //     NOTE: that 'cents_to_coin' happily returns members of
        //           Coin ... well... hmm, sorta
    }
}

fn cents_to_coin(cents: u32) -> Coin {
    match cents {
        1 => Coin::_Penny,
        5 => Coin::Nickle,
        10 => Coin::_Dime,
        25 => Coin::Quarter(UsState::Texas),
        _ => panic!("Unexpected coin value"),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::_Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickle => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("Ooh, a State quarter from {:?}.", state);
            25
        } //Coin::Quarter(_) ~~> 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
    // NOTE: "clippy" warns about my "manual implementation"
    //       and suggests 'x.map(|i| x+1)', instead
}
