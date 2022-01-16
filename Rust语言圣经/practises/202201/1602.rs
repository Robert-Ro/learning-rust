#[derive(Debug)]
struct User {
    email: String,
    active: bool,
    username: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("some@163.com"),
        active: true,
        username: String::from("some"),
        sign_in_count: 10,
    };
    println!("email: {}", user1.email);
    let mut user2 = User {
        email: String::from("some@163.com"),
        active: true,
        username: String::from("some"),
        sign_in_count: 10,
    };
    user2.email = String::from("some2@163.com");
    println!("user2's email: {}", user2.email);
    let user3 = User {
        email: String::from("any@163.com"),
        ..user1
    };
    println!("user3's email: {}", user3.email);
    // println!("user1's username: {}", user1.username); moved
    println!("user1's active: {}", user1.active);
    // println!("user1: {:?}", user1); // borrowed
}
