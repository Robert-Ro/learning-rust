#[allow(unused)]
use codewars::fundamental::*;
fn main() {
    let a = "sdf".chars().enumerate().map(|(i, c)| {
        c.to_string().to_uppercase() + c.to_string().to_lowercase().repeat(i).as_str()
    });
    println!("{:?}", a.collect::<Vec<_>>());
    println!("{}", 02 / 100 + 1);
}
