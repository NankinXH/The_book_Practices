#[derive(Debug)]
enum UsState {
    Alabama,
    Alsaska,
    // -- snip --
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // You can even run multiple lines of code in a match arm.
        // Coin::Penny => 1,
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // You can
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
fn main() {
    let first_coin = Coin::Quarter(UsState::Alabama);
    let filtered_coin = value_in_cents(first_coin);
    
    println!("{:?}", filtered_coin);
}
