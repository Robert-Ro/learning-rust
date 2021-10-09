use codewars::fundamental::fundamental::{
    dashatize, list_squared, multiplication_table, multiplication_table2,
};

fn main() {
    // test();
    multiplication_table(5);
    let v = multiplication_table2(3);
    println!("{:?}", v);
    let v = list_squared(1, 250);
    println!("{:?}", v);
    dashatize(234234);
}
