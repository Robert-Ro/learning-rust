use std::fs::File;
use std::io::prelude::*;
use std::usize;

fn lookup_ptr(addr: usize) -> std::io::Result<Option<String>> {
    let mut contents = String::new();
    File::open("/proc/self/maps")?.read_to_string(&mut contents)?;
    Ok(contents
        .split("\n")
        // Uncomment this if you are only interested in stack and heap
        // .filter(|l| l.contains("[heap]") || l.contains("[stack]")) // Find interesting segments
        .map(|l| l.split(" ").collect::<Vec<_>>()) // Split by tokens to extract address range & name
        // Convert to hex numbers
        .map(|l| {
            (
                l[0].split("-")
                    .take(2)
                    .map(|hexnr| usize::from_str_radix(hexnr, 16).unwrap()),
                l[l.len() - 1],
            )
        })
        // Convert to tuple
        .map(|(mut range_iter, name)| {
            (range_iter.next().unwrap(), range_iter.next().unwrap(), name)
        })
        // Find the right one
        .filter(|(from, to, _)| *from <= addr && *to > addr)
        .map(|(_, _, name)| name.to_owned())
        .next())
}

struct Employee<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    salary: i32,
    sales: i32,
    bonus: i32,
}

const BONUS_PERCENTAGE: i32 = 10;

fn get_bonus_percentage(salary: &i32) -> i32 {
    println!("=========== get_bonus_percentage ==============");
    println!(
        "get_bonus_percentage Salary {:p} : {:?}",
        salary,
        lookup_ptr(&salary as *const _ as usize).ok().unwrap()
    );
    let percentage = (salary * BONUS_PERCENTAGE) / 100;
    println!(
        "get_bonus_percentage Percentage {:p} : {:?}",
        &percentage,
        lookup_ptr(&percentage as *const _ as usize).ok().unwrap()
    );
    println!("=========== end get_bonus_percentage ==============");

    return percentage;
}

// salary is borrowed
fn find_employee_bonus(name: &str, salary: &i32, no_of_sales: i32) -> i32 {
    println!("=========== find_employee_bonus ==============");

    println!(
        "find_employee_bonus Name {:p} : {:?}",
        name,
        lookup_ptr(&name as *const _ as usize).ok().unwrap()
    );
    println!(
        "find_employee_bonus Name buffer {:p} : {:?}",
        name,
        lookup_ptr(name.as_ptr() as *const _ as usize).ok().unwrap()
    );
    println!(
        "find_employee_bonus salary {:p} : {:?}",
        salary,
        lookup_ptr(&salary as *const _ as usize).ok().unwrap()
    );
    println!(
        "find_employee_bonus no_of_sales {:p} : {:?}",
        &no_of_sales,
        lookup_ptr(&no_of_sales as *const _ as usize).ok().unwrap()
    );
    let bonus_percentage = get_bonus_percentage(salary);
    println!(
        "find_employee_bonus bonus_percentage {:p} : {:?}",
        &bonus_percentage,
        lookup_ptr(&bonus_percentage as *const _ as usize)
            .ok()
            .unwrap()
    );
    let bonus = bonus_percentage * no_of_sales;
    println!(
        "find_employee_bonus bonus {:p} : {:?}",
        &bonus,
        lookup_ptr(&bonus as *const _ as usize).ok().unwrap()
    );
    println!("=========== end find_employee_bonus ==============");

    return bonus;
}

fn main() {
    println!("=========== main ==============");

    // variable is declared as mutable
    let mut john = Employee {
        name: &format!("{}", "John"), // explicitly making the value dynamic
        salary: 5000,
        sales: 5,
        bonus: 0,
    };

    john.bonus = find_employee_bonus(john.name, &john.salary, john.sales);
    println!("Bonus for {} is {}", john.name, john.bonus);

    // Check the string object (stack as it is part of this function)
    println!(
        "John  {:p} : {:?}",
        &john,
        lookup_ptr(&john as *const _ as usize).ok().unwrap()
    );
    println!(
        "John name {:p} : {:?}",
        john.name,
        lookup_ptr(&john.name as *const _ as usize).ok().unwrap()
    );
    println!(
        "John name buffer {:p} : {:?}",
        &john.name,
        lookup_ptr(john.name.as_ptr() as *const _ as usize)
            .ok()
            .unwrap()
    );
    println!(
        "John salary {:p} : {:?}",
        &john.salary,
        lookup_ptr(&john.salary as *const _ as usize).ok().unwrap()
    );
    println!(
        "John bonus {:p} : {:?}",
        &john.bonus,
        lookup_ptr(&john.bonus as *const _ as usize).ok().unwrap()
    );
    println!("=========== end main ==============");
}

fn create_box(i: u32) {
    // Allocate a string on the heap
    let _var_1 = Box::new(format!("Hello {}", i));
    println!(
        "No.{} _var_1 {:p} : {:?}",
        i,
        &_var_1,
        lookup_ptr(_var_1.as_ptr() as *const _ as usize)
            .ok()
            .unwrap()
    );
    // `_var_1` is destroyed here, and memory gets freed
}

fn main2() {
    // Allocate an integer on the heap
    let _var_1 = Box::new(5u32);
    println!(
        "_var_1 {:p} : {:?}",
        &_var_1,
        lookup_ptr(_var_1.as_ref() as *const _ as usize)
            .ok()
            .unwrap()
    );
    // A nested scope:
    {
        // Allocate an integer on the heapcl
        let _var_2 = Box::new("Hello");
        println!(
            "_var_2 {:p} : {:?}",
            &_var_2,
            lookup_ptr(_var_2.as_ref() as *const _ as usize)
                .ok()
                .unwrap()
        );
        // `int_2` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for i in 0u32..3 {
        create_box(i);
    }

    // `int_1` is destroyed here, and memory gets freed
}
