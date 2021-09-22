fn main() {
    println!(
        "celsius tempture {}C° to fahrenheit tempture is: {}F°",
        32.0,
        celsius_to_fahrenheit(32.0)
    );
    println!(
        "fahrenheit tempture {}F° to celsius tempture is: {}C°",
        233.0,
        fahrenheit_to_celsius(233.0)
    );
    print!("the numbers of fibonacci(10): ");
    for elem in 1..11 {
        print!("{} ", fibonacci(elem));
    }
    println!("");
    print_songs();
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (9.0 / 5.0) * celsius + 32.0
}
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (5.0 / 9.0) * (fahrenheit - 32.0)
}
// F0 = 0
// F1 = 1
// FN = Fn-1 + Fn-2(n>=2)
fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
fn print_songs() {
    let arr = [
        "first", "second", "third", "four", "five", "six", "seven", "eigth", "nine", "ten",
        "eleven", "twelve",
    ];
    let len = arr.len() as i32;
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut index = 0;
    while index < len {
        println!(
            "On the {} day of Christmas my true love sent to me",
            arr[index as usize]
        );
        let mut j = index;
        if j == 0 {
            println!("{}.", gifts[j as usize]);
        } else {
            while j >= 0 {
                if j == 0 {
                    println!("And {}", gifts[j as usize].to_lowercase());
                } else {
                    println!("{},", gifts[j as usize]);
                }
                j -= 1;
            }
        }

        println!("");
        index += 1;
    }
}
