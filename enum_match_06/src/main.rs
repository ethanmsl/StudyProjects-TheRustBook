#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Texas,
    Vermont,
}
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
    // ^ Quarter along with info about which state is on it
}

fn main() {
    let nickle_coin = Coin::Nickle;
    println!("Value of a Nickle: {}", value_in_cents(nickle_coin));
    let texas_quarter = Coin::Quarter(UsState::Texas);
    println!("Value of a Quarter: {}", value_in_cents(texas_quarter));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Ooh, a State quarter from {:?}.", state);
            25
        }
        //Coin::Quarter(_) => 25,
    }
}
