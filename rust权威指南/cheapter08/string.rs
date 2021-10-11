fn main() {
    let mut s = String::new();
    let data = "initical contents";
    let s = data.to_string();
    let s = String::from("initial contents");
    let s = format!("initial contents");

    println!("{}", s);
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('a');
    let s2 = "bar";
    s.push_str(s2);
    println!("{}, {}", s, s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // s1所有权转移了, s2：解引用强制转换
                       // + 调用一个add方法
                       // fn add(&self, s:&str) ->String{}
                       // println!("{}, {}", s1, s3);
    println!("s2: {}, s3: {}", s2, s3);

    let s1 = &s3[..4];
    println!("{}", s1);
    let hello = "Здравствуйте";
    println!("{}", &hello[0..4]);
    // println!("{}", &hello[0..1]);
    for c in hello.chars() {
        println!("{}", c);
    }
    for c2 in "Здравствуйте".bytes() {
        println!("{}", c2);
    }
}
