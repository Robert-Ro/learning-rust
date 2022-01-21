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
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
