// 方法定义在某个结构体(或者枚举类型、trait对象)的上下文中，
// 并且它们的第一个参数永远都是self，用于指代调用该方法的结构体实例
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        // 1、获取所有权，2、借用不可变的所有权， 3、借用可变的所有权
        self.width * self.height
    }
    // 多个参数
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("the area of {:#?} is {}", rect, rect.area());
    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    let rect3 = Rectangle {
        width: 40,
        height: 20,
    };
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));
    let rect4 = Rectangle::square(10);
    println!("the area of {:#?} is {}", rect4, rect4.area());
}
