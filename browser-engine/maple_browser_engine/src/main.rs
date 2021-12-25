pub mod dom;
pub mod html;
use std::path::Path;
use std::{fs::File, io::Read};

fn main() {
    let content = read_source("./examples/text.html".to_string());
    // let content2:Vec<char> = content.clone().bytes().map(|b|b as char).collect();
    let nodes = html::parse(content.clone());
    println!("{:#?}", nodes);
    let node = html::Parser::parse(content);
    println!("{:#?}", node);
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(Path::new(&filename))
        .unwrap()
        .read_to_string(&mut str)
        .unwrap();
    str
}
