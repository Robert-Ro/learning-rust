fn main() {
    let s = String::from("Hello World");
    take_ownership(s);
    let x = 5;
    makes_copy(x);

    println!("x: {}", x); // s is valid

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("s1: {},  s3: {}", s1, s3); // s2 已经销毁了

    let s4 = String::from("Hello");
    let s5 = calculate_length(&s4); // 未转移S4的所有权
    println!("s4: {}, s5: {}", s4, s5);

    let mut s6 = String::from("Hello");
    let s7 = calculate_length2(&mut s6);
    // let s8 = &mut s6; // 数据竞争
    println!("s6: {}, s7: {}", s6, s7);
    {
        let s8 = &mut s6; // 数据竞争
        println!("s7: {}, s8: {}", s7, s8);
    }
}
//
fn take_ownership(some_thing: String) {
    println!("{}", some_thing);
}

// 原数据的副本
fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}

fn gives_ownership() -> String {
    let some_thing = String::from("hello");
    some_thing
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// 借用
fn calculate_length(s: &String) -> usize {
    // s.push_str(" world"); //不可变
    s.len()
}
// 可变引用
fn calculate_length2(s: &mut String) -> usize {
    s.push_str(" world");
    s.len()
}
