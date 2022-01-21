/**
 * if/while/for/loop/continue/break
 */
fn main() {
    let mut a = 5;
    if a > 5 {
        println!("{} is bigger than 5", a);
    }

    while a < 6 {
        println!("{} is smaller than 6", a);
        a = a + 1;
    }
    let b = loop {
        a = a + 1;
        if a > 100 {
            break a;
        }
    };
    println!("b is {}", b);
    for i in 1..100 {
        if i == 50 {
            continue;
        }
        println!("i is {}", i);
    }
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
    for (index, item) in vec.iter().enumerate() {
        println!("index is {}, value is {}", index, item);
    }
    for item in vec.iter().enumerate() {
        println!("item is {:?}", item);
        println!("index is {}, value is {}", item.0, item.1);
    }
}
