pub trait Summary<T> {
    fn summarize(&self) -> T;
}

pub struct Post<T> {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl<String> Summary<String> for Post {
    fn summarize(&self) -> String {
        format!("title: {}, author: {}", self.title, self.content)
    }
}

pub struct Weibo<T> {
    pub username: String,
    pub content: String,
}
impl<T> Summary<T> for Weibo {
    fn summarize(&self) -> T {
        format!("username: {}, content: {}", self.username, self.content)
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
}
