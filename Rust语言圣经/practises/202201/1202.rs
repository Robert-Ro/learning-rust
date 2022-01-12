fn main() {
    // let reference_to_nothing = dangle();
    let reference_to_nothing = dangle2();
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // this function's return type contains a borrowed value,该函数返回了一个借用的值
//        // but there is no value for it to be borrowed from但是已经找不到它所借用值的来源
// } // s离开了作用域并被丢弃。其内存被释放

fn dangle2() -> String {
    let s = String::from("hello");
    s // s的所有权被转移到外面的调用者
}
