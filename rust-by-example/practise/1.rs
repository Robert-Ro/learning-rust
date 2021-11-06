use std::fmt::{self, Formatter};

struct Strcture(i32);

impl fmt::Display for Strcture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Format: {}", self.0)
    }
}

impl fmt::Debug for Strcture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Debug: {}", self.0)
    }
}
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
      let x = &self.x + 10;
      let y = &self.y + 10;
        f.debug_struct("Point")
            .field("x", &x)
            .field("y", &y)
            .finish()
    }
}
fn main() {
    let s = Strcture(10);

    let form = format!("{}", s);
    println!("{}", form); // Format: 10
    let s = Strcture(10);
    println!("{:?}", &s); // Debug: 10
    println!("{}", s); // Format: 10
    let point = Point { x: 10, y: 10 };
    println!("{:?}", point);
}
