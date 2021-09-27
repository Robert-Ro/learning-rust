fn main() {
    /*********混合使用if let、else if、else if let和else********** */
    let faviorate_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "35".parse();

    if let Some(color) = faviorate_color {
        // 引入了新的变量age来存储Ok变体中的值，而它覆盖了右侧的同名变量
        println!("Using your faviorate_color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orangle as background color");
        }
    } else {
        println!("Using blue as background color");
    }
    /******while let使用举例*******/
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        // top 替换stack.pop()的值
        println!("{}", top);
    }
    /******for使用举例*******/
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{}: {}", index, value);
    }

    let (x, _, ..) = (1, 2, 3);
    println!("{}", x);

    let point = (3, 6);

    print_coordinates(&point);

    /************ */
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50!"),
        // 这个新的y的绑定能够匹配Some中的任意值，而x正是一个Some
        // 如果x不是Some(5)而是None，那么它会在前两个分支的模式匹配中匹配失败
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    // println!("x: {:?}, y: {}", x, y);
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("one through five, x: {}", x),
        _ => println!("something else"),
    }
    let x = Message::ChangeColor(Color::RGB(12, 12, 12));
    match x {
        Message::ChangeColor(Color::RGB(r, g, b)) => println!("RGB: {}, {}, {}", r, g, b),
        Message::ChangeColor(Color::HSV(h, s, v)) => println!("HSV: {}, {}, {}", h, s, v),
        _ => println!("None"),
    }
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let num = Some(5);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{} equal or bigger than five", x), // exec this
        None => println!(""),
    }
    // 使用外部变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}, ", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("x: {:?}, y: {}", x, y);

    let msg = Message2::Hello { id: 5 };
    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message2::Hello { id } => println!("Found some other id: {}, ", id),
    }
}

enum Color {
    RGB(i32, i32, i32),
    HSV(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("{}, {}", x, y);
}
enum Message2 {
    Hello { id: i32 },
}
