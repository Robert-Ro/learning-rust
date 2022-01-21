use std::f64::consts::PI;

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// 对象定义和方法定义是分离的，给予灵活性？

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, y, radius }
    }
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }
}
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    /// 关联函数
    /// 因为是函数，所以不能用.的方式来调用，我们需要用::来调用
    fn new(x: f64, y: f64) -> Rectangle {
        Rectangle {
            width: x,
            height: y,
        }
    }
    /// 方法名与结构名相同
    fn width(&self) -> f64 {
        self.width
    }
}

fn main() {
    let c = Circle::new(1.0, 1.0, 2.0);
    println!("{:?}'", c);
    println!("c area is {:?}", c.area());
    let r = Rectangle::new(2.0, 4.0);
    println!("the width of r is {}, {}", r.width(), r.width);
}
