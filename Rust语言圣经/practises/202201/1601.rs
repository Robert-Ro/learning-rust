fn main() {
    let tup: (i32, f64, u8) = (300, 6.0, 1);
    let (x, y, _) = tup;
    let x1 = tup.0;
    println!("x1: {}", x1);
    let (s, len) = calculate_length("hello".to_string());
    let (s, len) = calculate_length(String::from("hello"));
    println!("s: {}, len: {}", s, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
