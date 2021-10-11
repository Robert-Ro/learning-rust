// 字符串切片：指向字符串一部分内容的引用
fn main() {
    let mut s = String::from("Hello World");
    let worldIndex = first_word(&s);
    // s.clear();
    println!("{}", worldIndex);

    let hello = &s[..5]; // [开始索引..结束索引]
    let world = &s[6..];
    let whole = &s[..];
    println!("{}, {}, {}", hello, world, whole);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
