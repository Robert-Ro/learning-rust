// #[derive(Debug)]
enum S {
    First(String),
    Second(i32),
    Third(u8),
}
fn main() {
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3];
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    // vec1.push("2");//expected i32, found &str
    println!("the first value of {:#?}  is {}", vec1, vec1[0]);
    println!(
        "the length of {:#?} is {}, {}",
        vec1,
        vec1.len(),
        vec1.capacity() // ?
    );
    println!("the first value of {:#?}  is {}", vec2, vec2[0]);
    println!(
        "the length of {:#?} is {}, {}",
        vec2,
        vec2.len(),
        vec2.capacity() // ?
    );
    let vec3 = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &vec3[2];
    match vec3.get(2) {
        // 返回一个Option<&T>
        Some(third) => println!("{}", third),
        None => println!(""),
    };
    // let does_not_exist = &vec3[100]; // index out of bounds
    // println!("{:#?}", does_not_exist);
    let does_not_exist2 = vec3.get(100);
    println!("{:#?}", does_not_exist2);

    let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0]; // cannot borrow v as mutable because it is alos borrowed as immutable
    // vector扩容后，可能更换一个更大内存的空间来存储数据，但是first指向了v的一个元素的地址，这就造成不安全
    v.push(6);

    // println!("The first element is: {}", first);
    for elem in &v {
        println!("{}", elem);
    }
    let v = vec![S::First(String::from("hello")), S::Second(10), S::Third(8)];
    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut i: i32 = -1;
    for (elem, index) in &mut v.iter().enumerate() {
        println!("{}, {}", elem, index);
    }
    for elem in &mut v {
        *elem += 10;
        println!("{}", elem);
    }
    let res = match v.pop() {
        // Some(number) => println!("{}", number),
        // None => println!("there is no element in number"),
        Some(number) => number,
        None => -1,
    };
    println!("the pop ele is :{}", res);
}
