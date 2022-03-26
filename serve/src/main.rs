use std::fs::read_to_string;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // on the stack to hold the data that is read in
    stream.read(&mut buffer).unwrap(); // read and put the them in buffer
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let mut status_line;
    let contents;

    if buffer.starts_with(get) {
        status_line = "HTTP/1.1 200 OK";
        contents = read_to_string("./html/index.html");
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5)); // => Waiting (TTFB)
        status_line = "HTTP/1.1 200 OK";
        contents = read_to_string("./html/index.html");
    } else {
        status_line = "HTTP/1.1 404 NOT FOUND";
        contents = read_to_string("./html/404.html");
    }
    let contents = match contents {
        Ok(content) => content,
        Err(_) => {
            // TODO handle error
            status_line = "HTTP/1.1 500 internal error";
            read_to_string("./html/500.html").unwrap()
        }
    };
    stream
        .write(
            format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                contents.len(),
                contents
            )
            .as_bytes(),
        )
        .unwrap();
    stream.flush().unwrap(); // Flush this output stream, ensuring that all intermediately buffered contents reach their destination.
                             // println!("Request: {}", String::from_utf8_lossy(&buffer[..])); // lossy character
}
