fn main() {
    // scalar type
    let guess: u32 = "42".parse().expect("Not a number");
    println!("The value of guess is {}", guess);
    let decimal = 98_000;
    println!("The value of decimal is {}", decimal); // 98000
    let byte = b'A'; // u8
    println!("The value of byte is {}", byte); // 65

    let mut a: u8 = 233;
    a = 255; //error: literal out of range for `u8`
    println!("The value of a is {}", a);

    if (a > 20) {
        println!("{} is bigger than 20", a);
    }

    let c = 'z'; // char 4bytes U+0000-U+D7FF, U+E000-U+10FFFF, 无字符的概念
    let z = '😊';
    let heart_eyed_cat = '😈';
    println!(
        "the value of c is {}, z is {}, heart_eyed_cat is {}",
        c, z, heart_eyed_cat
    );

    // compound type
    // Tuple
    let tup: (i32, f64, u8) = (500, 200.23, 23);
    let (x, y, z) = tup;
    let first = tup.0;
    println!("The value of y is: {}, first is: {}", y, first);
    // Array
    let arr = [1, 2, 3, 4, 5, 6];
    println!("the first element in arr is {}", arr[0]);

    // println!("the 7th element in arr is {}", arr[7]); // index out of bounds
    //     有许多底
    // 层语言没有提供类似的检查，一旦尝试使用非法索引，你就会访问到
    // 某块无效的内存。在这种情况下，逻辑上的错误常常会蔓延至程序的
    // 其他部分，进而产生无法预料的结果。通过立即中断程序而不是自作
    // 主张地去继续运行，Rust帮助我们避开了此类错误。

    for ele in arr.iter() {
        println!("The element is {}", ele);
    }
    let iter = &["a", "b", "c"];
    for elem in iter {
        println!("{}", elem);
    }
}
