fn main() {
    let x = (-42.0f32).sqrt();
    // assert_eq!(x, x);
    if x.is_nan() {
        println!("未定义的数学行为");
    }
    // range
    for i in 1..=5 {
        println!("i: {:?}", i);
    }
    for i in 'a'..'z' {
        println!("i: {:?}", i);
    }
    println!("13.14.round: {:?}", 13.14f32.round());
    let x = '中';
    println!("'中' size: {}", std::mem::size_of_val(&x));
    println!("x = {}", x);
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
