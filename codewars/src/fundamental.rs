pub mod fundamental {
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
fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(list_squared2(m, n), exp)
}
#[test]
fn basics_list_squared() {
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(42, 250, vec![(42, 2500), (246, 84100)]);
    testing(300, 600, vec![]);
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
