use serve::ThreadPool;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3080").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool::execute(|| {
            // handle_connection(stream);
        })
    }
}

// fn handle_connection(mut stream)-> {

// }
// https://doc.rust-lang.org/book/title-page.html