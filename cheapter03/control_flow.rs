fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // if number { // expected `bool`, found integer
    //     println!("number was three");
    // }
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // rust需要在编译时确定number的具体类型，如果Rust能够使用运行时确定number类型，
    // 那么它就不得不记录变量所有可能出现的类型，这会是的编译器的实现更加复杂，并丧失许多安全代码保障。
    // let number2 = if condition { 5 } else { "six" }; // expected integer, found `&str`
    println!("The value of number is: {}", number);
    // loop
    // rust provice 3 types: loop, while and for
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("the current counter is: {}", counter);
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    println!("\n");
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
    println!("\n");
    let a = [10, 20, 30, 40, 50];
    for elem in a.iter() {
        println!("the value is: {}", elem);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
