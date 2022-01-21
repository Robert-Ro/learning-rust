enum Direction {
    East,
    West,
    North,
    South,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}
fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North")
        }
        _ => println!("West"),
    };
    value_in_cents(Coin::Dime);
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 1),
        Action::ChangeColorRGB(255, 255, 255),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color to '(r:{}, g:{}, b: 0)', 'b' has been ignored",
                    r, g
                );
            }
        }
    }
    if let Some(e) = Some(3) {
        println!("e is {}", e);
    }
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let foo = v.iter().filter(|x| matches!(x, MyEnum::Bar)); //FIXME
                                                             // .collection::<Vec<MyEnum>>();
    println!("{:?}", foo);
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z'|'a'..='z'));
    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));

    let age = Some(30);

    println!("匹配前age is {:?}", age);
    if let Some(age) = age {
        println!("匹配出来的age是{:?}", age);
    }
    println!("匹配后age is {:?}", age);
    println!("****************************");
    let age1 = Some(30);

    println!("匹配前age is {:?}", age1);
    if let Some(age2) = age1 {
        println!("匹配出来的age是{:?}", age2);
    }
    println!("匹配后age is {:?}", age1);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}", six, none);

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(pop) = stack.pop() {
        println!("{}", pop);
    }

    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("{}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("{}", id)
        }
        Message::Hello { id } => {
            println!("{}", id)
        }
    } 
}

enum Message {
    Hello { id: i32 },
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
