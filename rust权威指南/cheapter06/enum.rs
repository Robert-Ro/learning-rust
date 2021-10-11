// enum IpAddrKind {
//     V4,
//     V6,
// }
// 直接将数据附加到了枚举的每个变体中， 不需要额外地使用结构体
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
//STL
// struct Ipv4Addr {}
// struct Ipv6Addr {}
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }
#[derive(Debug)]
enum Message {
    Quit,                       // 无任何关联数据
    Move { x: i32, y: i32 },    //包含了一个匿名结构体
    Write(String),              // 包含了一个String
    ChangeColor(i32, i32, i32), //包含了3个i32值
}
impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
    fn call2() {
        println!("call2");
    }
}
fn main() {
    // let home = IpAddr {
    //     kind: IpAddrKind::V4, // :: 访问枚举值
    //     address: String::from("127.0.0.1"),
    // };
    // let lookup = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let lookback = IpAddr::V6(String::from("::1"));

    let home = IpAddr::V4(127, 0, 0, 1);
    let lookback = IpAddr::V6(String::from("::1"));
    let m = Message::Write(String::from("Hello"));
    m.call(); // Write("Hello")
    Message::call2(); // call2
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;
}
