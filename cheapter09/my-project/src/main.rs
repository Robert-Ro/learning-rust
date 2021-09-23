use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];

    // v[99];
    // RUST_BACKTRACE=1 cargo run => 输出详细的调用栈信息
    let file_path = "../../readme.md";
    let f = File::open(file_path);
    match f {
        Ok(file) => println!("{:?}", file),
        //         File { file handle文件句柄
        //     fd: 3,
        //     path: "/home/liutsing/maple/learning-rust/readme.md",
        //     read: true,
        //     write: false,
        // }
        Err(e) => println!("Err: {}", e),
    }
    let file_path = "hello.txt";
    let f = File::open(file_path);
    let file = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("try to create file, but Error: {:?}", e),
            },
            other_error => panic!("there was a problem opening the file: {:?}", other_error),
        },
    };
    println!("{:?}", file);

    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            });
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
    println!("{:?}", f); // Ok(File { fd: 4, path: "/home/liutsing/maple/learning-rust/cheapter09/my-project/hello.txt", read: true, write: false })
                         // 当Result的返回值是Ok变体时，unwrap就会返回Ok内部的值。
                         // 而当Result的返回值是Err变体时，unwrap则会替我们调用`panic!`宏

    // let f = File::open("hello2.txt").unwrap();

    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // match read_username_from_file("hello.text") {
    //     Ok(s) => println!("s: {}", s),
    //     Err(e) => panic!("e: {:?}", e),
    // }
    // read_username_from_file("he").expect("no such file: he");
    read_username_from_file("hello.txt").unwrap();
    read_username_from_file2("hello2.txt").unwrap();
    Ok(())
}

fn read_username_from_file(file_path: &str) -> Result<String, io::Error> {
    let f = File::open(file_path);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
// 使用?来简化错误传播
fn read_username_from_file2(file: &str) -> Result<String, io::Error> {
    let mut f = File::open(file)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
// 链式
fn read_username_from_file3(file: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(file)?.read_to_string(&mut s)?;
    Ok(s)
}
// 读取文件简化操作
fn read_username_from_file4(file_path: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_path)
}


