fn main() {
    let mut s = String::from("HELLO");
    take_ownship(&s);
    let x = 5;
    makes_copy(x);

    println!("x: {}, s: {}", x, s); // value borrowed here after move
    s.push_str(", world");
    println!("{}", s);
    let s2 = s;
    println!("{}", s2);
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); //s2's ownership被转移了
    println!("{}", s2);
}
/**
 * 引用
 */
fn take_ownship(some_thing: &str) {
    println!("{}", some_thing);
}
/**
 * 基本类型，默认实现了Copy trait
 */
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
/**
 * 给予所有权
 */
fn gives_ownership() -> String {
    let some_thing = String::from("hello"); // some_string 进入作用域.
    some_thing // 返回 some_string 并移出给调用的函数
}
/**
 * 获取所有权并将所有权返回出去
 */
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string
}
