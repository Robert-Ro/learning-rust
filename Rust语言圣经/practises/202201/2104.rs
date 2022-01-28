use std::cmp::PartialOrd;

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("first ele");
    }
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
fn largest<T: Copy + PartialOrd>(list: &[T]) -> T {
    // 为泛型添加限制,类似ts中的T extends xx
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// largest函数
///
/// 不需要强求每个参数都实现Copy trait.使用引用的方式
///
/// * `list` - 集合T.
fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest3 = &list[0];
    for item in list.iter() {
        if item > largest3 {
            largest3 = item;
        }
    }
    &largest3
}

fn main() {
    let m = Message::Quit;
    m.call();
    let a = add(1i8, 2i8);
    println!("{}", a);

    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("the largest in number_list is {}", result);

    let result = largest2(&number_list);
    println!("the largest2 in number_list is {}", result);

    let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("the largest char is {}", result);

    let p = Point { x: 3, y: 4.0 };
    println!("p is {:?}", p);
    Point::new(1, 2.0f32);
    println!("p.x is {}", p.x());
    let p2 = p.mixup(Point { x: 'a', y: 4.0f64 });
    println!("{:?}", p2);

    let p = Point { x: 2f32, y: 3f32 };

    println!("{}", p.distance_from_origin());
    // println!("{}", p2.distance_from_origin()); E0423, E0599
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);
    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}
/// N 基于值的泛型参数
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

impl<T: std::fmt::Display, U: std::fmt::Display> Point<T, U> {
    fn new(x: T, y: U) {
        println!("x is {}, y is {}", x, y);
    }
    fn x(&self) -> &T {
        &self.x
    }
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<V, W> {
        Point {
            x: other.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
