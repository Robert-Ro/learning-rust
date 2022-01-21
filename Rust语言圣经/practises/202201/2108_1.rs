trait Draw2 {
    fn draw(&self) -> String;
}
impl Draw2 for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}
impl Draw2 for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}
fn draw1(x: Box<dyn Draw2>) {
    println!("{}", x.draw());
}

fn draw2(x: &dyn Draw2) {
    println!("{}", x.draw());
}

fn main() {
    let x = 1.1f64;
    let y = 8u8;

    draw1(Box::new(x));
    draw1(Box::new(y));

    draw2(&x);
    draw2(&y);
}
