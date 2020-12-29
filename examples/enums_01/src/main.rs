enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater => 25,
    }
}

fn main() {

}
