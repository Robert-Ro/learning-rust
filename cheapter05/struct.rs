struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let user: User = User {
        username: String::from("lilei"),
        email: String::from("lilei@gmail.com"),
        sign_in_count: 10,
        active: true,
    };
    // user.sign_in_count = 2; // error[E0594]: cannot assign to `user.sign_in_count`, as `user` is not declared as mutable
    println!("user.name: {}", user.username);
    let mut user2 = User {
        // 所有属性都是可变的
        email: String::from("a@gmail.com"),
        username: String::from("username"),
        sign_in_count: 4,
        active: false,
    };
    user2.active = true;
    println!("user2.active: {}", user2.active);
    let user = build_user(
        String::from("hanmeimei"),
        String::from("hanmeimei@gmail.com"),
    );
    println!("user.username: {}", user.username);
    let mut user = build_user(String::from("lili"), String::from("lili@gmail.com"));
    user.sign_in_count = 11;
    println!("user.sign_in_count: {}", user.sign_in_count);
    let user2 = User {
        email: String::from("test@163.com"),
        ..user
    };
    println!("user2.username: {}", user2.username);

    // 元组结构体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let blank = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 10,
        active: false,
    }
}
