use std::fs::File;

fn main() {
    let v = vec![1, 2, 3];

    // v[99];

    match open_file() {
        Ok(f) => {
            println!("f: {:?}", f);
        }
        Err(e) => {
            println!("err: {:?}", e);
        }
    }
}

fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    let mut f = File::open("hello.txt")?;
    Ok(f)
}
