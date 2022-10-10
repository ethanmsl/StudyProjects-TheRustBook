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
