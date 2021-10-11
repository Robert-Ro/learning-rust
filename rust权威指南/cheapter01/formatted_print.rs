/**
* Printing is handled by a series of macros defined in std::fmt some of which include:
* format!: write formatted text to String
* print!: Same as format! but the text is printed to the console (io::stdout).
* println!: Same as print! but a newline is appended.
* eprint!: Same as format! but the text is printed to the standard error(io::stderr).
* eprintln!: Same as eprint! but a newline is appended.
*/
fn main() {
    // In general, the `{}` will be automatically replaced with any arguments, These will be stringified.
    format!("{} text", "text"); // not to console
    print!("{} text", "text"); // no newline
    print!("\n");
    println!("{} days", 31); // with new line
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    // Special formatting can be specified after a `:`.
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );
    println!("the binary of {} is {:b}", 10, 10); // the binary of 10 is 1010
    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:0>width$}", number = 1, width = 6);
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // #[allow(dead_code)]
    // struct Structure(i32);
    // println!("This struct `{:?}` won't print...", Structure(3)); FIXME
    let pi = 3.1415;
    println!("Pi is roughly {}", pi);
    // doc examples
    println!("{:?}", (3, 4));
    println!("{:04}", 42);
    println!("{:#?}", (100, 200));
    // positional parameters
    println!("{1} {} {0} {}", 1, 2);
    // named prameters
    println!("{argument}", argument = "test");
    println!("{name} {}", 1, name = 2);
    println!("{a} {c} {b}", a = "a", b = "b", c = 3);
    // formatting parameters
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);
    println!("Hello {1:0$}!", 5, "x");
    println!("Hello {:width$}!", "x", width = 5);
}
