use std::fmt::Display;
fn main() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    // 例子2
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // 例子3
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest2(string1.as_str(), string2.as_str(), "");
    }
    println!("The longest string is {}", result);

    //
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
// this function's return type contains a borrowed value,
// but the signature does not say whether it is borrowed from `a` or `b`
// 我们需要给返回类型标注一个
// 泛型生命周期参数，因为Rust并不能确定返回的引用会指向x还是指向y
// longest函数的定义指定了签名中所有的引用都必须拥有相同的生命周期'a
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    //expected named lifetime parameter
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
// FIXME 编译通过
fn longest2<'a, 'b>(a: &'a str, b: &'b str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
// fn longest3<'a>(a: &str, b: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str() // 输出不与输入相关联
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}
