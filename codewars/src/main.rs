use codewars::fundamental::fundamental::{
    dashatize, list_squared, multiplication_table, multiplication_table2,
};
use codewars::rank_up::rank_up::{anagrams, high, order, parse, perimeter};

fn main() {
    // test();
    multiplication_table(5);
    let v = multiplication_table2(3);
    println!("{:?}", v);
    let v = list_squared(1, 250);
    println!("{:?}", v);
    dashatize(234234);
    // parse("iiisdoso");
    perimeter(4);
    order("is2 Thi1s T4est 3a");
    let high = high("man i need a taxi up to ubud");
    let a = anagrams("abab", &["carer".to_string(), "racer".to_string()]);
}
