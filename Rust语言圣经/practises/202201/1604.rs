#[derive(Debug)]
enum PokeSuit {
    Clubs,
    Spades,
    Dimamonds,
    Hearts,
}
#[derive(Debug)]
struct PokeCard {
    suit: PokeSuit,
    value: u8,
}
#[derive(Debug)]
enum PokeCard2 {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}
enum Message {
    Quit,                       // 没有任何关联数据
    Move { x: i32, y: i32 },    // 包含一个匿名结构体
    Write(String),              // 包含一个String字符串
    ChangeColor(i32, i32, i32), // 包含三个i32
}

fn main() {
    let heart = PokeSuit::Hearts;
    let diamond = PokeSuit::Dimamonds;
    print_suit(heart);
    print_suit(diamond);
    let c1 = PokeCard {
        suit: PokeSuit::Clubs,
        value: 1,
    };
    let c2 = PokeCard {
        suit: PokeSuit::Dimamonds,
        value: 12,
    };
    println!("{:?}", c1);
    let c3 = PokeCard2::Spades(5);
    let c4 = PokeCard2::Diamonds(13);
    println!("{:?}", c3);
    println!("{:?}", c4);
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 255, 0);
    let some_number = Some(5);
    let some_string = Some(String::from("hello"));
    let number: Option<i32> = None; // 需要制定Option<T>是什么类型，只通过 None 值无法推断出 Some 成员保存的值的类型
    let x = 32 + some_number.unwrap();
    let x2 = 32 + some_number.expect("hello");
    println!("{}, {}", x, x2);
}

fn print_suit(card: PokeSuit) {
    println!("{:?}", card);
}
