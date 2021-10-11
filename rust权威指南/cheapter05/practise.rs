#[derive(Debug)] // derive注解来派生的trait，它们可以为自定义的类型增加许多有用的功能
struct Rectangles {
    width: i32,
    height: i32,
}

fn main() {
    let rect = Rectangles {
        width: 30,
        height: 50,
    };
    println!("the area of {:#?} is {}", rect, get_rect_area(&rect)); // pretty print rect
}
// 借用rect的所有权
fn get_rect_area(rect: &Rectangles) -> i32 {
    rect.width * rect.height
}
