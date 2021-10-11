#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents2(coin: Coin2) -> u32 {
    match coin {
        Coin2::Penny => {
            println!("Penny");
            1
        }
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:#?}", state);
            25
        }
    }
}
fn main() {
    let a = Coin::Penny;
    let res = value_in_cents(a);
    println!("{}", res);
    let b = Coin2::Quarter(UsState::Alabama);
    println!("{}", value_in_cents2(b));
    let five = Some(5);
    let size = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", five); // Some(5)
    let v = 7u8;
    match v {
        1 => println!("One"),
        3 => println!("Three"),
        5 => println!("Five"),
        7 => println!("Seven"),
        _ => (),
    };
    let v2 = Some(10u8);
    if let Some(3) = v2 {
        println!("Three");
    } else {
        println!("others");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
