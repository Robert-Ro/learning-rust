use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // assert_eq!(5, y); // no implementation for `{integer} == &{integer}`

    let x = 5;
    let y = Box::new(5);
    let y2 = MyBox::new(5);
    println!("{:p}", &x);
    println!("{:p}", y);
    // println!("{:p}", y2);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *y2);

    let m = MyBox::new(String::from("Rust"));
    //&m &MyBox<String>
    // defer &String
    // defer &str
    hello(&m);

    hello("Rust");

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c); //手动清理
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointer created.");
    // auto invoke drop
    // Dropping CustomSmartPointer with data: other stuff
    // Dropping CustomSmartPointer with data: my stuff
}

fn hello(name: &str) {
    println!("{}", name);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// impl<T> Pointer for MyBox<T> {}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: '{}'", self.data);
    }
}
