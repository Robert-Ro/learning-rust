fn main() {
    let s = String::from("HELLO");
    take_ownship(&s);
    let x = 5;
    makes_copy(x);

    println!("x: {}, s: {}", x, s); // value borrowed here after move
}
/**
 * 引用
 */
fn take_ownship(some_thing: &str) {
    println!("{}", some_thing);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_thing = String::from("hello"); // some_string 进入作用域.
    some_thing // 返回 some_string 并移出给调用的函数
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string
}
