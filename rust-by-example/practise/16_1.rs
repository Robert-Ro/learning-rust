#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self; // 取出值
        Centimeters(inches as f64 * 2.54)
    }
}
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);
    let foot = Inches(12);
    println!("One foot equals {:?}", foot);
    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "Smaller"
    } else {
        "Bigger"
    };
    println!("One foot is {} than one meter.", cmp);
}
