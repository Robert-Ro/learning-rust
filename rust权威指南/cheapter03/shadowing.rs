fn main() {
    let x = 12; // unused
    let x = 13; // shadow previous value
    println!("The value of x is {}", x);
    let mut spaces = "     ";
    let len = spaces.len();
    println!("The length of {} is {}", spaces, len);
}
