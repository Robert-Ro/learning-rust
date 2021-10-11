fn main() {
    println!("Hello World!");
    another_function();
    another_function1(3);
    // another_function1(); // this function takes 1 argument but 0 arguments were supplied
    // another_function1(300); //literal out of range for `u8`
    let t = another_function2();
    println!("the result of another_function2() is: {}", t);
    // let res2 = another_function3(); //error[E0277]: `()` doesn't implement `std::fmt::Display`
    // println!("The value of another_function3() is: {}", res2);
}

fn another_function() {
    println!("Another function");
}

// parameters must provide the type for parameters
fn another_function1(x: u8) {
    println!("The value of x is: {}", x);
}
// fn statement vs fn expression
// rust based on expression

fn another_function2() -> u8 {
    //statement no returns
    let y = 6;
    // let x = (let z = 7); // Error: variable declaration using `let` is a statement
    // expresion: 1:会计算出某个值来作为结果。
    // 2: 表达式本身也可以作为语句的一部分
    // 3、调用函数是表达式，调用宏也是表达式， 创建新作用域的花括号（{}）也是表达式
    y + 7
}
fn another_function3() {
    let y = 6;
    y + 4; // 返回了一个空元组，也就是上面描述中的()
}
// returns
fn five() -> i32 {
    5 // 作为结果返回的表达式
}
