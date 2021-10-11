use crate::List::{Cons, Nil};
use std::fmt::Debug;
use std::ops::{Deref, Drop};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // *(y.deref())
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    drop(m);
    println!("******************");
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let b = Cons(3, Rc::clone(&a)); // 不会执行数据的深度拷贝操作，只增加引用计数

    println!("count after creating a = {}", Rc::strong_count(&a)); //1
    let b = Cons(6, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); //2
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); //3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); //2
    println!("******************");
}


fn hello(name: &str) {
    println!("Hello, {}!", name);
}
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0 //不希望获得MyBox<T>内部值的所有权
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Drop droped");
    }
}
