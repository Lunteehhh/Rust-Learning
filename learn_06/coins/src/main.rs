#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cent(coin: Coin) -> u8{
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    let x = Coin::Nickel;
    let y = Coin::Penny;
    let z = Coin::Quarter(UsState::Alaska);
    
    println!("Nickel: {}", value_in_cent(x));
    println!("Penny: {}", value_in_cent(y));
    println!("Quarter: {}", value_in_cent(z));
}
