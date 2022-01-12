fn main() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');
    assert_eq!(s, "hello, world!");

    let mut s = "hello, world".to_string();
    s.push('!');
    assert_eq!(s, "hello, world!");

    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello, world!");
    // println!("{}", s1); s1已经被转移所有权到s3上了

    let s = String::from("hello, world");
    string2str(&s);
    string2str(&s[..]);
    string2str(s.as_str());
    for c in "中国人".chars() {
        println!("{}", c);
    }
    for b in "中国人".bytes() {
        println!("{}", b);
    }
}

fn string2str(s: &str) {
    println!("{}", s);
}
