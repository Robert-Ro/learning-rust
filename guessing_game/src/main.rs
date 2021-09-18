use rand::Rng; // trait
use std::cmp::Ordering; // enum
use std::io; // prelude
             // @link: https://kaisery.github.io/trpl-zh-cn/ch02-00-guessing-game-tutorial.html
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            // enum
            Ok(num) => num,
            Err(e) => {
                eprintln!("Err: {}! Your should enter valid number!", e);
                continue;
            }
        };
        println!("Your guessed {}", guess);
        match guess.cmp(&secret_number) {
            // enum
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
