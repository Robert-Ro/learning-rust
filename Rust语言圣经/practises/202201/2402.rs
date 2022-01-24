fn main() {
    let a = i8::MAX;
    println!("{}", a);
    let b: u32 = a as u32;
    println!("{}", b);

    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize; // 内存地址转变为一个整数
    let second_address = first_address + 4; // 内存地址平移4个字节, i32占4个字节
    let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    unsafe {
        println!("p2: {:?}", *p2); // dereference of raw pointer is unsafe and requires unsafe function or block
    }
}
