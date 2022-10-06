enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn main() {
    let nickle_coin = Coin::Nickle;
    println!("Value of a Nickle: {}", value_in_cents(nickle_coin));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
