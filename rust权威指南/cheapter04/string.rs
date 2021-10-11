fn main() {
    let mut str = String::from("Hello");
    str.push_str(" world");
    println!("{}", str);
    let str2 = str;
    // println!("{}", str); //value borrowed here after move
    // let str3 = str.clone();
    // println!("{} {}", str3, str);
    let x = 10;
    let y = x;
}
