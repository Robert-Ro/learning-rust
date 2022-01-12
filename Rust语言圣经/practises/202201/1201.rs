fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("s: {}", s);
    let r1 = &s;
    let r2 = &s;

    println!("{} and {}", r1, r2); // r1, r2的作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
} // r3作用域在这里结束

fn change(some_thing: &mut String) {
    some_thing.push_str(", world");
}
