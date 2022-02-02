#![allow(dead_code)]
#![allow(unused_variables)]
use regex::*;
use std::collections::{BTreeSet, HashMap};
use std::iter::FromIterator;
use std::{cmp, vec};

pub fn likes(names: &[&str]) -> String {
    match names {
        [] => format!("no one likes this"),
        [a] => format!("{} likes this", a),
        [a, b] => format!("{} and {} like this", a, b),
        [a, b, c] => format!("{}, {} and {} like this", a, b, c),
        [a, b, rest @ ..] => format!("{}, {} and {} others like this", a, b, rest.len()),
    }
}
pub fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    (1..=len)
        .into_iter()
        .map(|oi| (1..=len).map(|ii| ii * oi).collect())
        .collect()
}
pub fn multiplication_table2(len: usize) -> Vec<Vec<usize>> {
    (1..=len)
        .map(|n| {
            //NOTE 步长n，取len个
            (n..).step_by(n).take(len).collect()
        })
        .collect()
}
pub fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    (m..=n)
        .map(|x| (1..=x).filter(|y| x % y == 0).collect::<Vec<_>>()) // 获取除数
        .filter(|x| {
            let temp: u64 = x.into_iter().map(|y| y.pow(2)).sum();
            is_sqrt_ok(temp) //过滤整数是完全平方数
        })
        .map(|x| {
            let lst = x.last().unwrap();
            let s: u64 = x.iter().map(|y| y.pow(2)).sum();
            (*lst, s)
        })
        .collect()
}
pub fn is_sqrt_ok(data: u64) -> bool {
    ((data as f64).sqrt() as u64).pow(2) == data
}
pub fn list_squared2(m: u64, n: u64) -> Vec<(u64, u64)> {
    //FIXME 看不懂
    (m..=n)
        .map(|i| {
            (
                i,
                (1..=(i as f32).sqrt() as u64)
                    .filter_map(|d| {
                        let q = i / d;
                        if q * d == i {
                            Some(d * d + if q == d { 0 } else { q * q })
                        } else {
                            None
                        }
                    })
                    .sum(),
            )
        })
        .filter(|t| (t.1 as f64).sqrt().fract() == 0.0)
        .collect::<Vec<_>>()
}
pub fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut v: Vec<i32> = arr.to_vec().into_iter().filter(|x| x % 2 == 1).collect();
    v.sort_by(|a, b| b.cmp(a));
    let mut res: Vec<i32> = Vec::new();
    let origin = arr.to_vec().clone();
    for i in 0..arr.len() {
        if arr.get(i).unwrap() % 2 == 1 {
            res.push(v.pop().unwrap());
        } else {
            res.push(*origin.get(i).unwrap());
        }
    }
    res
}
pub fn is_odd(i: i32) -> bool {
    i % 2 != 0
}
pub fn sort_array2(arr: &[i32]) -> Vec<i32> {
    let mut odds = arr
        .iter()
        .cloned()
        .filter(|&x| is_odd(x))
        .collect::<Vec<i32>>();
    odds.sort();
    let mut odds_iter = odds.into_iter();
    arr.iter()
        .map(|&x| {
            if is_odd(x) {
                odds_iter.next().unwrap()
            } else {
                x
            }
        })
        .collect()
}
pub fn dashatize(n: i64) -> String {
    Regex::new(r"[02468]+|[13579]")
        .unwrap()
        .find_iter(&n.abs().to_string())
        .for_each(|c| {
            println!("{:?}", c);
            // Match { text: "234234", start: 0, end: 1 }
            // Match { text: "234234", start: 1, end: 2 }
            // Match { text: "234234", start: 2, end: 4 }
            // Match { text: "234234", start: 4, end: 5 }
            // Match { text: "234234", start: 5, end: 6 }
        });
    Regex::new(r"[02468]+|[13579]")
        .unwrap()
        .find_iter(&n.abs().to_string())
        .map(|m| m.as_str())
        .collect::<Vec<_>>()
        .join("-")
    // n.abs()
    //     .to_string()
    //     .replace("1", "-1-")
    //     .replace("3", "-3-")
    //     .replace("5", "-5-")
    //     .replace("7", "-7-")
    //     .replace("9", "-9-")
    //     .replace("--", "-")
    //     .trim_matches('-')
    //     .to_string()
}
/**
 * Write a function that will return the count of distinct case-insensitive alphabetic characters and numeric digits that occur more than once in the input string.
 * @params text The input string can be assumed to contain only alphabets (both uppercase and lowercase) and numeric digits.
 */
pub fn count_duplicates(text: &str) -> u32 {
    /************solution one**************/
    let map =
        text.chars()
            .map(|c| c.to_lowercase().to_string())
            .fold(HashMap::new(), |mut map, c| {
                let count = map.entry(c).or_insert(0);
                *count += 1;
                return map;
            });
    let mut count = 0;
    for (_v, i) in map.iter() {
        if *i > 1 {
            count += 1;
        }
    }
    println!("{}", count);
    /***********solution two************/
    let mut map: HashMap<char, u32> = HashMap::new();
    for c in text.to_lowercase().chars() {
        let count = map.entry(c).or_default();
        *count += 1;
    }
    map.values().filter(|&&v| v > 1).count() as u32
}
/**
 * NOTE 数据量大时，O(n)会超出时间，这时就需要用怎么降低时间复杂度到O(logn)或O(1)
 */
pub fn _bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if h <= window {
        return -1;
    }
    let mut init = h;
    let mut count: f64 = 0f64;

    loop {
        count += 1f64; // top -> down
        init *= bounce;
        if init < window {
            break;
        };
        count += 1f64; // down -> top
    }
    count as i32
}
pub fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    // NOTE 采用数学思维
    if !(h > 0. && 0. < bounce && bounce < 1. && window < h) {
        -1
    } else {
        (window / h).log(bounce).ceil() as i32 * 2 - 1
    }
}
// pub fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
//     // your code
//     Some((1, 1))
// }

// A divisibility rule is a shorthand way of determining whether a given integer is divisible
// by a fixed divisor without performing the division, usually by examining its digits.
// https://www.codewars.com/kata/564057bc348c7200bd0000ff/train/rust
pub fn thirt(n: i64) -> i64 {
    let res = _thirt(n);
    if res == n {
        res
    } else {
        _thirt(res)
    }
}
fn _thirt(n: i64) -> i64 {
    let mods = vec![1, 10, 9, 12, 3, 4];
    /******use zip******/
    n.to_string()
        .chars()
        .into_iter()
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .zip(mods.iter().cycle())
        .map(|(d, p)| d * *p)
        .sum::<u32>() as i64
    /******use for cycle******/
    // let mut res = 0;
    // for (index, char) in n.to_string().chars().into_iter().rev().enumerate() {
    //     res += char.to_digit(10).unwrap() * mods[index % 6];
    // }
    // res as i64
}
/// https://www.codewars.com/kata/554b4ac871d6813a03000035/train/rust
pub fn high_and_low(numbers: &str) -> String {
    // solution 1
    // let strs: Vec<&str> = numbers.split(" ").collect();
    // let init: i128 = strs[0].parse().unwrap();
    // let mut res: (i128, i128) = (init, init);
    // for i in strs {
    //     let n: i128 = i.parse().unwrap();
    //     println!("{}", n);
    //     if n > res.0 {
    //         res.0 = n;
    //     } else if n <= res.1 {
    //         res.1 = n;
    //     }
    // }
    // solution 2
    let f = |(max, min), x| (cmp::max(max, x), cmp::min(min, x));
    let res = numbers
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .fold((i32::min_value(), i32::max_value()), f);
    format!("{} {}", res.0, res.1)
}

/// https://www.codewars.com/kata/5583090cbe83f4fd8c000051/train/rust
pub fn digitize(n: u64) -> Vec<u8> {
    // NOTE split("") 前后存在空格，parse所以报错
    // solution1
    // let mut res = n
    //     .to_string()
    //     .split("")
    //     .filter(|c| !c.is_empty())
    //     .map(|x| x.parse().unwrap())
    //     .collect::<Vec<u8>>();

    // res.reverse();
    // res
    // solution 2
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .rev()
        .collect::<Vec<u8>>()
}

fn number(bus_stops: &[(i32, i32)]) -> i32 {
    bus_stops
        .iter()
        .fold(0, |accum, item| accum + item.0 - item.1)
}
fn reverse(str: &str) -> String {
    str.chars().rev().collect()
    // String::from_iter(str.chars().rev())
}

fn reverse_words(str: &str) -> String {
    str.to_string()
        .split(" ")
        .map(|c| reverse(c))
        .collect::<Vec<String>>()
        .join(" ")
}

fn disemvowel(s: &str) -> String {
    // String::from_iter(
    //     s.chars()
    //         .filter(|c| !"aeiou".contains(c.to_ascii_lowercase())),
    // )
    s.chars()
        .filter(|c| !"aeiou".contains(c.to_ascii_lowercase()))
        .collect()
}

fn string_to_number(s: &str) -> i32 {
    s.parse().unwrap()
}

fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data.into_iter()
        .map(|c| {
            if c.0 >= 55 && c.1 >= 7 {
                "Senior"
            } else {
                "Open"
            }
            .to_string()
        })
        .collect()
}

pub fn dig_pow(n: i64, p: i32) -> i64 {
    // let str = n.to_string().chars().fold((p as u32, 0), |acc, curr| {
    //     let value: i64 = i64::pow(curr.to_digit(10).unwrap() as i64, acc.0) + acc.1;
    //     (acc.0 + 1, value)
    // });
    // let result = str.1;

    let result: i64 = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(|(index, d)| i64::pow(d as i64, index as u32 + p as u32))
        .sum();

    match result % n {
        0 => result / n,
        _ => -1,
    }
}

fn dna_to_rna(dna: &str) -> String {
    // dna.chars()
    //     .map(|c| if c == 'T' { 'U' } else { c })
    //     .collect()

    dna.replace("T", "U")
}

fn xo(string: &'static str) -> bool {
    let map: HashMap<String, u32> = string.chars().fold(HashMap::new(), |mut map, c: char| {
        let count = map.entry(c.to_ascii_lowercase().to_string()).or_insert(0);
        *count += 1;
        map
    });
    map.get("x") == map.get("o")
    // string.chars().fold(0, |a, c| match c {
    //     'x' | 'X' => a + 1,
    //     'o' | 'O' => a - 1,
    //     _ => a,
    // }) == 0
}

fn no_space(x: String) -> String {
    x.replace(" ", "")
}

fn alphabet_position(text: &str) -> String {
    // solution1
    //     text.split_whitespace()
    //         .map(|word| {
    //             word.chars()
    //                 .map(|cur| {
    //                     if cur.is_ascii_alphabetic() {
    //                         (cur.to_ascii_lowercase()) as u32 - 'a' as u32 + 1
    //                     } else {
    //                         0
    //                     }
    //                     .to_string()
    //                 })
    //                 .filter(|c| c != "0")
    //                 .collect::<Vec<String>>()
    //                 .join(" ")
    //         })
    //         .collect::<Vec<String>>()
    //         .join(" ")
    // }
    // solution2
    text.to_lowercase()
        .chars()
        .filter(|c| c >= &'a' && c <= &'z')
        .map(|c| (c as u32 - 96).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.len() == 0 {
        return vec![];
    }
    input.iter().fold(vec![0, 0], |mut acc, curr| {
        if curr > &0 {
            acc[0] += 1
        } else {
            acc[1] += curr
        }
        acc
    })
}

fn binary_slice_to_number(slice: &[u32]) -> u32 {
    // solution1
    // slice.into_iter().rev().fold([0, 0], |acc, d| {
    //     [acc[0] + 1, acc[1] + u32::pow(2, acc[0]) * d]
    // })[1]
    // solution2
    slice.iter().fold(0, |acc, bit| (acc << 1) | bit) // most clever answer
}

fn longest(a1: &str, a2: &str) -> String {
    // solution1
    // let mut s: Vec<char> = [a1, a2]
    //     .concat()
    //     .chars()
    //     .fold(String::new(), |mut acc, curr| {
    //         if !acc.contains(curr) {
    //             acc.push(curr);
    //         }
    //         acc
    //     })
    //     .chars()
    //     .collect();
    // s.sort_by(|&a, &b| (a as u32).cmp(&(b as u32)));
    // s.iter().collect()
    // solution2
    a1.chars() //NOTE  chain BTreeSet
        .chain(a2.chars())
        .collect::<BTreeSet<char>>()
        .iter()
        .collect()
}

pub fn accum(s: &str) -> String {
    // solution1
    // s.chars()
    //     .fold((0, Vec::new()), |mut acc: (usize, Vec<String>), curr| {
    //         if acc.0 > 0 {
    //             let aa = [
    //                 vec![curr.to_ascii_uppercase()],
    //                 [curr.to_ascii_lowercase()].repeat(acc.0),
    //             ]
    //             .concat()
    //             .iter()
    //             .collect();
    //             acc.1.push(aa);
    //         } else {
    //             acc.1.push(vec![curr.to_ascii_uppercase()].iter().collect());
    //         }
    //         return (acc.0 + 1, acc.1);
    //     })
    //     .1
    //     .join("-")
    // solution2
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            // NOTE map
            c.to_string().to_uppercase() + c.to_string().to_lowercase().repeat(i).as_str()
        })
        .collect::<Vec<String>>()
        .join("-")
}

fn abbrev_name(name: &str) -> String {
    name.split_whitespace()
        .into_iter()
        .map(|c| {
            // solution1
            // c.chars().collect::<Vec<char>>()[0]
            //     .to_string()
            //     .to_ascii_uppercase()
            // solution2
            c.chars().nth(0).unwrap().to_string().to_ascii_uppercase()
        })
        .collect::<Vec<String>>()
        .join(".")
}

fn century(year: u32) -> u32 {
    match year % 100 {
        0 => year / 100,
        _ => year / 100 + 1,
    }
}

fn bool_to_word(value: bool) -> &'static str {
    match value {
        true => "Yes",
        false => "No",
    }
}

fn descending_order(x: u64) -> u64 {
    // solution1
    // let mut nums = x
    //     .to_string()
    //     .chars()
    //     .map(|c| c.to_digit(10).unwrap())
    //     .collect::<Vec<u32>>();
    // nums.sort_by(|a, b| b.cmp(a));
    // nums.iter()
    //     .map(|c| c.to_string())
    //     .collect::<Vec<String>>()
    //     .join("")
    //     .parse()
    //     .unwrap()
    // solution2
    let mut nums = x.to_string().chars().collect::<Vec<char>>();
    nums.sort_by(|a, b| b.cmp(a)); // 字符串可以直接匹配
    String::from_iter(nums).parse::<u64>().unwrap()
}

///  the time since midnight in milliseconds.
///
/// * `h` - hour
/// * `m` - minute
/// * `s` - second
///
fn past(h: i32, m: i32, s: i32) -> i32 {
    h * 3600 * 1000 + m * 60 * 1000 + s * 1000
}

fn greet(language: &str) -> &str {
    let mut map: HashMap<&str, &str> = HashMap::with_capacity(17);
    let data = vec![
        ("english", "Welcome"),
        ("czech", "Vitejte"),
        ("danish", "Velkomst"),
        ("dutch", "Welkom"),
        ("estonian", "Tere tulemast"),
        ("finnish", "Tervetuloa"),
        ("flemish", "Welgekomen"),
        ("french", "Bienvenue"),
        ("german", "Willkommen"),
        ("irish", "Failte"),
        ("italian", "Benvenuto"),
        ("latvian", "Gaidits"),
        ("lithuanian", "Laukiamas"),
        ("polish", "Witamy"),
        ("spanish", "Bienvenido"),
        ("swedish", "Valkommen"),
        ("welsh", "Croeso"),
    ];
    data.iter().for_each(|c| {
        map.entry(c.0).or_insert(c.1);
    });

    match map.get(&language) {
        Some(t) => t,
        _ => "Welcome",
    }
}

fn count_sheep(sheep: &[bool]) -> u8 {
    // sheep.iter().fold(0, |mut acc, curr| {
    //     match curr {
    //         true => acc += 1,
    //         false => {}
    //     };
    //     acc
    // })
    sheep.iter().filter(|&&c| c).count() as u8
}

fn find_short(s: &str) -> u32 {
    // solution1
    // s.split_whitespace().fold((0, 0), |mut acc, curr|{
    //     if acc.0 == 0 {
    //         acc.1 = curr.len()
    //     }else {
    //        if acc . 1 > curr.len() {  acc.1 =  curr.len() };
    //     }
    //     (1, acc.1)
    // }).1.try_into().unwrap()
    // solution2
    s.split_whitespace().map(str::len).min().unwrap() as u32
}

fn count_sheep2(n: u32) -> String {
    // solution1
    // let mut res = String::new();
    // for i in 1..=n {
    //     res.push_str(format!("{} sheep...", i).as_str());
    // }
    // res
    // solution2
    (1..=n).map(|x| format!("{} sheep...", x)).collect()
}
fn solution(word: &str, ending: &str) -> bool {
    word.contains(ending)
}
fn greet2() -> String {
    // format!("hello world!")
    String::from("hello world!")
}
fn to_alternating_case(s: &str) -> String {
    // solution1
    //   s.chars().map(|a|{
    //     match a.is_ascii_lowercase() {
    //         true => a.to_ascii_uppercase().to_string(),
    //         false => a.to_ascii_lowercase().to_string()
    //       }
    //   }).collect::<Vec<String>>().join("")
    // solution2
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_uppercase() {
            result.extend(c.to_lowercase());
        } else {
            result.extend(c.to_uppercase());
        }
    }
    result
}
fn litres(time: f64) -> i32 {
    (time as i32) / 2
}
fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    // solution1
    //    let rest =  cap - on - wait;
    //    match rest >= 0 {
    //      true =>  0,
    //    false =>    0- rest
    //    }
    // solution2
    (on + wait - cap).max(0)
}
fn wave(s: &str) -> Vec<String> {
    // (0..s.len())
    //     .map(|c| {
    //         s.chars()
    //             .enumerate()
    //             .map(move |(i, char)| match i == c {
    //                 true => char.to_ascii_uppercase(),
    //                 false => char,
    //             })
    //             .collect::<String>()
    //     })
    //     .collect::<Vec<String>>()
    s.char_indices()
        .map(|(i, c)| s[..i].to_string() + &c.to_uppercase().to_string() + &s[i + 1..])
        .filter(|w| w != s)
        .collect()
}
fn odd_or_even(numbers: Vec<i32>) -> String {
    match numbers.iter().sum::<i32>() % 2 == 0 {
        true => "even".to_string(),
        false => "odd".to_string(),
    }
}
fn gimme(input_array: [i32; 3]) -> usize {
    let min = input_array.iter().min().unwrap();
    let max = input_array.iter().max().unwrap();
    input_array.iter().position(|n| min < n && n < max).unwrap() // 在迭代器中搜索元素，并返回其索引。
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

// fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
//     todo!();
// }
fn nb_dig(n: i32, d: i32) -> i32 {
    // your code
    0
}
fn update_light(current: &str) -> String {
    match current {
        "green" => "yellow",
        "yellow" => "red",
        "red" => "green",
        _ => panic!(),
    }
    .into()
}
fn are_you_playing_banjo(name: &str) -> String {
    match name.starts_with("R") || name.starts_with("r") {
        // 按索引取0/全转变为lowercase再比对
        true => format!("{} plays banjo", name),
        false => format!("{} does not play banjo", name),
    }
}
fn get_char(c: i32) -> char {
    // unsafe { std::char::from_u32_unchecked(c as u32) }
    c as u8 as char
}
fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
    // code here
    // if arr.len() == 2 {
    //     return None;
    // }
    // arr.iter().fold((0, 0), |acc, cur| {
    //     if acc.0 == 0 {
    //         acc.1 = cur;
    //     } else {
    //         if acc.1 - cur != 1 {
    //             return Some(cur);
    //         } else {
    //             acc.1 = cur;
    //         }
    //     }
    //     (acc.0 + 1, acc.1)
    // });
    // None
    arr.windows(2).find(|s| s[0] + 1 != s[1]).map(|s| s[1]) //NOTE  window find
}
fn is_divide_by(number: i32, a: i32, b: i32) -> bool {
    number.abs() % a.abs() == 0 && number.abs() % b.abs() == 0
}
fn find_smallest_int(arr: &[i32]) -> i32 {
    // your code here
    *arr.iter().min().unwrap()
}
//*****************************************************************/
#[cfg(test)]
fn _testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(list_squared2(m, n), exp)
}
fn _testequal(h: f64, bounce: f64, window: f64, exp: i32) -> () {
    assert_eq!(bouncing_ball(h, bounce, window), exp);
}
#[test]
fn test_likes() {
    assert_eq!(likes(&[]), "no one likes this");
    assert_eq!(likes(&["Peter"]), "Peter likes this");
    assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
    assert_eq!(
        likes(&["Max", "John", "Mark"]),
        "Max, John and Mark like this"
    );
    assert_eq!(
        likes(&["Alex", "Jacob", "Mark", "Max"]),
        "Alex, Jacob and 2 others like this"
    );
}
#[test]
fn test_multiplication_table() {
    assert_eq!(multiplication_table(3), [[1, 2, 3], [2, 4, 6], [3, 6, 9]]);
}

#[test]
fn basics_list_squared() {
    _testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    _testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    _testing(42, 250, vec![(42, 2500), (246, 84100)]);
    _testing(300, 600, vec![]);
}

#[test]
fn test_sort_array() {
    assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
    assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
    assert_eq!(sort_array(&[]), []);
}
#[test]
fn test_basic_dashatize() {
    assert_eq!(dashatize(274), "2-7-4");
    assert_eq!(dashatize(5311), "5-3-1-1");
    assert_eq!(dashatize(86320), "86-3-20");
    assert_eq!(dashatize(974302), "9-7-4-3-02");
}

#[test]
fn test_weird_dashatize() {
    assert_eq!(dashatize(0), "0");
    assert_eq!(dashatize(-1), "1");
    assert_eq!(dashatize(-28369), "28-3-6-9");
}
#[test]
fn test_abcde() {
    assert_eq!(count_duplicates("abcde"), 0);
}

#[test]
fn test_abcdea() {
    assert_eq!(count_duplicates("abcdea"), 1);
}

#[test]
fn test_indivisibility() {
    assert_eq!(count_duplicates("indivisibility"), 1);
}

#[test]
fn tests_bouncing_ball() {
    _testequal(3.0, 0.66, 1.5, 3);
    _testequal(30.0, 0.66, 1.5, 15);
    _testequal(40.0, 0.4, 10.0, 3);
    _testequal(10.0, 0.6, 10.0, -1);
}

// fn gap_testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
//     assert_eq!(gap(g, m, n), exp)
// }

// #[test]
// fn basics_gap() {
//     gap_testing(2, 100, 110, Some((101, 103)));
//     gap_testing(4, 100, 110, Some((103, 107)));
//     gap_testing(6, 100, 110, None);
//     gap_testing(8, 300, 400, Some((359, 367)));
// }
#[test]
fn test_thirt() {
    assert_eq!(thirt(8529), 79);
    assert_eq!(thirt(85299258), 31);
    assert_eq!(thirt(5634), 57);
}
#[test]
fn test_high_and_low() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
    assert_eq!("5 1", high_and_low("1 2 3 4 5"));
    assert_eq!("5 -3", high_and_low("1 2 -3 4 5"));
    assert_eq!("9 -5", high_and_low("1 9 3 4 -5"));
}
#[test]
fn test_digitize() {
    assert_eq!(digitize(35231), vec![1, 3, 2, 5, 3]);
    assert_eq!(digitize(0), vec![0]);
}
#[test]
fn test_number() {
    assert_eq!(number(&[(10, 0), (3, 5), (5, 8)]), 5);
    assert_eq!(
        number(&[(3, 0), (9, 1), (4, 10), (12, 2), (6, 1), (7, 10)]),
        17
    );
    assert_eq!(
        number(&[(3, 0), (9, 1), (4, 8), (12, 2), (6, 1), (7, 8)]),
        21
    );
}
#[test]
fn test_reverse_words() {
    assert_eq!(
        reverse_words("The quick brown fox jumps over the lazy dog."),
        "ehT kciuq nworb xof spmuj revo eht yzal .god"
    );
    assert_eq!(reverse_words("apple"), "elppa");
    assert_eq!(reverse_words("a b c d"), "a b c d");
    assert_eq!(
        reverse_words("double  spaced  words"),
        "elbuod  decaps  sdrow"
    );
}
#[test]
fn test_disemvowel() {
    assert_eq!(
        disemvowel("This website is for losers LOL!"),
        "Ths wbst s fr lsrs LL!"
    );
}
#[test]
fn test_string_to_number() {
    assert_eq!(string_to_number("1234"), 1234);
    assert_eq!(string_to_number("605"), 605);
    assert_eq!(string_to_number("1405"), 1405);
    assert_eq!(string_to_number("-7"), -7);
}

#[test]
fn works_on_random() {
    use rand::prelude::*;
    let mut rng = thread_rng();
    for _ in 0..5 {
        let num: i32 = rng.gen();
        let input = num.to_string();
        assert_eq!(string_to_number(&input), num);
    }
}
#[test]
fn test_open_or_senior() {
    assert_eq!(
        open_or_senior(vec![(45, 12), (55, 21), (19, -2), (104, 20)]),
        vec!["Open", "Senior", "Open", "Senior"]
    );
    assert_eq!(
        open_or_senior(vec![(3, 12), (55, 1), (91, -2), (54, 23)]),
        vec!["Open", "Open", "Open", "Open"]
    );
}

fn _test_dig_pow(n: i64, p: i32, exp: i64) -> () {
    println!(" n: {:?};", n);
    println!("p: {:?};", p);
    let ans = dig_pow(n, p);
    println!(" actual:\n{:?};", ans);
    println!("expect:\n{:?};", exp);
    println!(" {};", ans == exp);
    assert_eq!(ans, exp);
    println!("{};", "-");
}

#[test]
fn test_dig_pow() {
    _test_dig_pow(89, 1, 1);
    _test_dig_pow(92, 1, -1);
    _test_dig_pow(46288, 3, 51);
}
#[test]
fn test_dna_to_rna() {
    assert_eq!(dna_to_rna("TTTT"), "UUUU");
    assert_eq!(dna_to_rna("GCAT"), "GCAU");
}
#[test]
fn test_xo() {
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);
}
#[test]
fn test_no_space() {
    assert_eq!(
        "8j8mBliB8gimjB8B8jlB",
        no_space("8 j 8   mBliB8g  imjB8B8  jl  B".to_string())
    );
    assert_eq!(
        "88Bifk8hB8BB8BBBB888chl8BhBfd",
        no_space("8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd".to_string())
    );
    assert_eq!("8aaaaaddddr", no_space("8aaaaa dddd r     ".to_string()));
    assert_eq!(
        "jfBmgklf8hg88lbe8",
        no_space("jfBm  gk lf8hg  88lbe8 ".to_string())
    );
    assert_eq!("8jaam", no_space("8j aam".to_string()));
}
#[test]
fn test_alphabet_position() {
    assert_eq!(
        alphabet_position("The sunset sets at twelve o' clock."),
        "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
    );
    assert_eq!(
        alphabet_position("The narwhal bacons at midnight."),
        "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
    );
}
#[test]
fn test_count_positives_sum_negatives() {
    let test_data1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
    let expected1 = vec![10, -65];
    assert_eq!(count_positives_sum_negatives(test_data1), expected1);
}
#[test]
fn test_binary_slice_to_number() {
    assert_eq!(binary_slice_to_number(&vec![0, 0, 0, 1]), 1);
    assert_eq!(binary_slice_to_number(&vec![0, 0, 1, 0]), 2);
    assert_eq!(binary_slice_to_number(&vec![1, 1, 1, 1]), 15);
    assert_eq!(binary_slice_to_number(&vec![0, 1, 1, 0]), 6);
}

fn testing(s1: &str, s2: &str, exp: &str) -> () {
    println!("s1:{:?} s2:{:?}", s1, s2);
    println!("{:?} {:?}", longest(s1, s2), exp);
    println!("{}", longest(s1, s2) == exp);
    assert_eq!(&longest(s1, s2), exp);
}

#[test]
fn basic_tests() {
    testing("aretheyhere", "yestheyarehere", "aehrsty");
    testing(
        "loopingisfunbutdangerous",
        "lessdangerousthancoding",
        "abcdefghilnoprstu",
    );

    assert_eq!(
        accum("ZpglnRxqenU"),
        "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu"
    );
    assert_eq!(
        accum("NyffsGeyylB"),
        "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb"
    );
    assert_eq!(
        accum("MjtkuBovqrU"),
        "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu"
    );
    assert_eq!(
        accum("EvidjUnokmM"),
        "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm"
    );
    assert_eq!(
        accum("HbideVbxncC"),
        "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc"
    );

    assert_eq!(abbrev_name("Sam Harris"), "S.H");
    assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
    assert_eq!(abbrev_name("Evan Cole"), "E.C");
    assert_eq!(abbrev_name("P Favuzzi"), "P.F");
    assert_eq!(abbrev_name("David Mendieta"), "D.M");

    assert_eq!(century(1705), 18);
    assert_eq!(century(1900), 19);
    assert_eq!(century(1601), 17);
    assert_eq!(century(2000), 20);
    assert_eq!(century(89), 1);

    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);

    assert_eq!(past(0, 1, 1), 61000);
    assert_eq!(past(1, 1, 1), 3661000);
    assert_eq!(past(0, 0, 0), 0);
    assert_eq!(past(1, 0, 1), 3601000);
    assert_eq!(past(1, 0, 0), 3600000);

    assert_eq!(greet("english"), "Welcome");
    assert_eq!(greet("dutch"), "Welkom");
    assert_eq!(greet("IP_ADDRESS_INVALID"), "Welcome");
    assert_eq!(greet(""), "Welcome");
    assert_eq!(greet("swelsh"), "Welcome");

    assert_eq!(count_sheep(&[false]), 0);
    assert_eq!(count_sheep(&[true]), 1);
    assert_eq!(count_sheep(&[true, false]), 1);

    assert_eq!(
        find_short("bitcoin take over the world maybe who knows perhaps"),
        3
    );
    assert_eq!(
        find_short("turns out random test cases are easier than writing out basic ones"),
        3
    );
    assert_eq!(
        find_short("lets talk about javascript the best language"),
        3
    );
    assert_eq!(
        find_short("i want to travel the world writing code one day"),
        1
    );
    assert_eq!(find_short("Lets all go on holiday somewhere very cold"), 2);
    assert_eq!(find_short("Let's travel abroad shall we"), 2);

    assert_eq!(count_sheep2(0), "");
    assert_eq!(count_sheep2(1), "1 sheep...");
    assert_eq!(count_sheep2(2), "1 sheep...2 sheep...");
    assert_eq!(count_sheep2(3), "1 sheep...2 sheep...3 sheep...");

    assert_eq!(true, solution("abc", "c"));
    assert_eq!(false, solution("strawberry", "banana"));

    assert_eq!("HELLO WORLD", to_alternating_case("hello world"));
    assert_eq!("hello world", to_alternating_case("HELLO WORLD"));
    assert_eq!("HELLO world", to_alternating_case("hello WORLD"));
    assert_eq!("hEllO wOrld", to_alternating_case("HeLLo WoRLD"));
    assert_eq!(
        "Hello World",
        to_alternating_case(&to_alternating_case("Hello World")[..])
    );
    assert_eq!("12345", to_alternating_case("12345"));
    assert_eq!("1A2B3C4D5E", to_alternating_case("1a2b3c4d5e"));
    assert_eq!(
        "sTRING.tOaLTERNATINGcASE",
        to_alternating_case("String.ToAlternatingCase")
    );

    assert_eq!(litres(2.), 1);
    assert_eq!(litres(1.4), 0);
    assert_eq!(litres(12.3), 6);
    assert_eq!(litres(0.82), 0);
    assert_eq!(litres(11.8), 5);
    assert_eq!(litres(1787.), 893);
    assert_eq!(litres(0.), 0);

    assert_eq!(enough(10, 5, 5), 0, "enough(10, 5, 5) should return 0");
    assert_eq!(
        enough(100, 60, 50),
        10,
        "enough(100, 60, 50) should return 10"
    );
    assert_eq!(enough(20, 5, 5), 0, "enough(20, 5, 5) should return 0");

    let expect = ["Hello", "hEllo", "heLlo", "helLo", "hellO"];
    assert_eq!(wave("hello"), expect);

    let expect = [
        "Codewars", "cOdewars", "coDewars", "codEwars", "codeWars", "codewArs", "codewaRs",
        "codewarS",
    ];
    assert_eq!(wave("codewars"), expect);

    let expect: [&str; 0] = [];
    assert_eq!(wave(""), expect);

    let expect = [
        "Two words",
        "tWo words",
        "twO words",
        "two Words",
        "two wOrds",
        "two woRds",
        "two worDs",
        "two wordS",
    ];
    assert_eq!(wave("two words"), expect);

    let expect = [" Gap ", " gAp ", " gaP "];
    assert_eq!(wave(" gap "), expect);

    assert_eq!(gimme([2, 3, 1]), 0);
    assert_eq!(gimme([-2, -3, -1]), 0);
    assert_eq!(gimme([5, 10, 14]), 1);
    // TODO
    // [-5, -10, -14] 1
    // [-1614572743, -1857284097, -1913689197] 1
    // use Direction::{self, *};
    // let a = [North, South, South, East, West, North, West];
    // assert_eq!(dir_reduc(&a), [West]);

    // let a = [North, West, South, East];
    // assert_eq!(dir_reduc(&a), [North, West, South, East]);

    assert_eq!(get_char(55), '7');
    assert_eq!(get_char(56), '8');
    assert_eq!(get_char(57), '9');
    assert_eq!(get_char(58), ':');
    assert_eq!(get_char(59), ';');
    assert_eq!(get_char(60), '<');
    assert_eq!(get_char(61), '=');
    assert_eq!(get_char(62), '>');
    assert_eq!(get_char(63), '?');
    assert_eq!(get_char(64), '@');
    assert_eq!(get_char(65), 'A');

    assert_eq!(first_non_consecutive(&vec![1, 2, 3, 4, 6, 7, 8]), Some(6));
    assert_eq!(first_non_consecutive(&vec![1, 2, 3, 4, 5, 6, 7, 8]), None);
    assert_eq!(first_non_consecutive(&vec![4, 6, 7, 8, 9, 11]), Some(6));
    assert_eq!(first_non_consecutive(&vec![4, 5, 6, 7, 8, 9, 11]), Some(11));
    assert_eq!(first_non_consecutive(&vec![31, 32]), None);
    assert_eq!(first_non_consecutive(&vec![-3, -2, 0, 1]), Some(0));
    assert_eq!(first_non_consecutive(&vec![-5, -4, -3, -1]), Some(-1));

    assert_eq!(is_divide_by(8, 2, 4), true);
    assert_eq!(is_divide_by(12, -3, 4), true);
    assert_eq!(is_divide_by(8, 3, 4), false);
    assert_eq!(is_divide_by(48, 2, -5), false);
    assert_eq!(is_divide_by(-100, -25, 10), true);
    assert_eq!(is_divide_by(10000, 5, -3), false);
    assert_eq!(is_divide_by(4, 4, 2), true);
    assert_eq!(is_divide_by(5, 2, 3), false);
    assert_eq!(is_divide_by(-96, 25, 17), false);
    assert_eq!(is_divide_by(33, 1, 33), true);

    assert_eq!(find_smallest_int(&[34, 15, 88, 2]), 2);
    assert_eq!(find_smallest_int(&[34, -345, -1, 100]), -345);
}
