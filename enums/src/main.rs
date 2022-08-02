#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn count_not(coin_not_quarter: Coin) -> u8 {
    let mut count = 0;
    // match coin_not_quarter {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // };
    if let Coin::Quarter(state) = coin_not_quarter {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    count
}

fn main() {
    let p: Coin = Coin::Quarter(UsState::Alaska);
    let cents = value_in_cents(p);
    println!("cents is {:?}!", cents);

    let p1: Coin = Coin::Penny;
    let count_p1 = count_not(p1);
    println!("count is {}!", count_p1);
}
