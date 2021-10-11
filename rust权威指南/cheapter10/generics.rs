fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let p2 = Point2 { x: 1, y: 2.0 };

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// fn largest(arr: &[i32]) -> i32 {
//     let mut max_number = arr[0];
//     for &elem in arr.iter() {
//         if elem > max_number {
//             max_number = elem;
//         }
//     }
//     max_number
// }
// 可以被用于任何实现了PartialOrd与Copy这两个trait的泛型
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &elem in list.iter() {
        if elem > largest {
            largest = elem;
        }
    }
    largest
}
// FIXME 不再需要Clone或Copy来进行trait约束
// fn largest2<T: PartialOrd>(list: &[T]) -> &T {
//     let &mut max_value = list[0];
//     for &elem in list.iter() {
//         if elem > max_value {
//             max_value = elem;
//         }
//     }
//     return max_value;
// }
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
struct Point2<T, U> {
    x: T,
    y: U,
}
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
