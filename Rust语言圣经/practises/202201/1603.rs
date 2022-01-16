#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User<'a> {
    username: &'a str,
    email: &'a str,
    sign_in_count: u32,
    active: bool,
}

fn main() {
    let f1 = File {
        name: String::from("f1.text"),
        data: Vec::new(),
    };
    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
    let black: Color = Color(0, 0, 0);
    let user = User {
        email: "some@163.com",
        username: "some",
        active: true,
        sign_in_count: 10,
    };
}
