use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    // 1、 生成随机数
    let mut rng = thread_rng();
    let secret_number = rng.gen_range(1..101);

    loop {
        // 2、 获取键盘输入
        println!("Please input number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取输入");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("e: {}", e);
                continue;
            }
        };
        // 3、比较两个数
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too great"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
