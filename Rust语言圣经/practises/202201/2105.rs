use std::convert::TryInto;
use std::fmt::Display;
pub trait Summary {
    fn summary_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more... from {}", self.summary_author())
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    // fn summarize(&self) -> String {
    //     format!("title: {}, author: {}", self.title, self.content)
    // }
    fn summary_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("username: {}, content: {}", self.username, self.content)
    }
    fn summary_author(&self) -> String {
        format!("@{}", self.username)
    }
}
fn main() {
    let p = Post {
        title: String::from("title"),
        author: String::from("author"),
        content: String::from("content"),
    };
    println!("{}", p.summarize());

    let w = Weibo {
        username: String::from("username"),
        content: String::from("content"),
    };
    println!("{}", w.summarize());

    notify(&w);
    notify2(&w);

    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap(); // TryInto下的方法
    if a < b_ {
        println!("a is less than b");
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify3<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn returns_summartizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from("m1 max太厉害了，电脑再也不卡了"),
    }
}
