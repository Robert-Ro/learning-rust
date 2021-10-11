fn main() {
    // let x = 5; immutable
    // println!("The value of x is: {}", x);
    // x = 6; // error
    // println!("The value of x is {}", x);
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is {}", x);
    let str = String::from("hello");
    let str = str.len();
    println!("The length of str is {}", str);

    const CONST1: i32 = 10; // Error must provide a type
                            // CONST1 = 11; Error: cannot assign to this expression
    println!("The value of CONST1 is {}", CONST1);
    // const mut cons2 = 12; Error: cannot be mutable
    // cons2 = 13;
    // println!("The value of cons is {}", cons2);
    {
        println!("The value of CONST1 is {}", CONST2); // can be accessed
        const CONST2: u32 = 100;
    }
    // println!("The value of CONST2 is {}", CONST2); // error[E0425]: cannot find value `CONST2` in this scope
}
