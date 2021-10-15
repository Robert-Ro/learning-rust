pub mod fundamental {
    use std::collections::HashMap;

    use regex::*;

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
    fn is_sqrt_ok(data: u64) -> bool {
        ((data as f64).sqrt() as u64).pow(2) == data
    }
    //FIXME
    pub fn list_squared2(m: u64, n: u64) -> Vec<(u64, u64)> {
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
    fn is_odd(i: i32) -> bool {
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
        let map = text.chars().map(|c| c.to_lowercase().to_string()).fold(
            HashMap::new(),
            |mut map, c| {
                let count = map.entry(c).or_insert(0);
                *count += 1;
                return map;
            },
        );
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
    // NOTE 采用数学思维
    pub fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
        if !(h > 0. && 0. < bounce && bounce < 1. && window < h) {
            -1
        } else {
            (window / h).log(bounce).ceil() as i32 * 2 - 1
        }
    }
}

#[cfg(test)]
use super::fundamental::fundamental::*;
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

use crate::fundamental::fundamental::list_squared2;
fn _testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(list_squared2(m, n), exp)
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
use crate::fundamental::fundamental::bouncing_ball;
fn _testequal(h: f64, bounce: f64, window: f64, exp: i32) -> () {
    assert_eq!(bouncing_ball(h, bounce, window), exp);
}

#[test]
fn tests_bouncing_ball() {
    _testequal(3.0, 0.66, 1.5, 3);
    _testequal(30.0, 0.66, 1.5, 15);
    _testequal(40.0, 0.4, 10.0, 3);
    _testequal(10.0, 0.6, 10.0, -1);
}
