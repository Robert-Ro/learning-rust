use std::convert::TryInto;

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
        *p2 += 2;
        println!("p2: {:?}", *p2); // dereference of raw pointer is unsafe
                                   // and requires unsafe function or block
    }
    let mut values2: [i32; 2] = [2, 4];
    let p3: *mut i32 = values2.as_mut_ptr();
    let first_addres = p3 as usize;
    let second_address = first_addres + 10;
    let p4 = second_address as *mut i32;
    unsafe {
        *p4 += 1;
        println!("p3: {:?}", *p4);
    }

    let a: u8 = 10;
    let b: u16 = 1500;
    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string()); // "out of range integral type conversion attempted"
            0
        }
    };
    println!("{}", b_);
    if a < b_ {
        println!("a:{} is less than b:{}", a, b_);
    }
}
