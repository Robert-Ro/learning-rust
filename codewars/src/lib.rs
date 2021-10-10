pub mod fundamental;
use std::collections::HashSet;

pub fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut res = arr_a
        .iter()
        .filter(|&a| arr_b.iter().any(|&b| b.contains(a)))
        .map(|x| x.to_string())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<String>>();

    res.sort();
    res
}
pub fn song_decoder(song: &str) -> String {
    song.split("WUB")
        // .filter(|x| x.to_string().len() > 0)
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()
        .join(" ")
}
// 计算发生了乘法的次数
pub fn persistence(num: u64) -> u64 {
    if num < 10 {
        return 0;
    }
    let mut num2 = num;
    let mut count: u64 = 0;
    while num2 >= 10 {
        count += 1;
        num2 = num2
            .to_string()
            .split("")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .fold(1, |sum, val| sum * val)
    }
    count
}
pub fn persistence2(num: u64) -> u64 {
    if num < 10 {
        return 0;
    }
    1 + persistence2(num.to_string().chars().fold(1 as u64, |sum, val| {
        sum * val.to_string().parse::<u64>().unwrap()
    }))
}
pub fn validate_pin(pin: &str) -> bool {
    // let len = pin.len();
    // if len != 4 && len != 6 {
    //     return false;
    // }
    // pin.chars().all(|s| match s.to_string().parse::<u8>() {
    //     Ok(_) => true,
    //     _ => false,
    // })
    pin.chars().all(|c| c.is_digit(10)) && (pin.len() == 4 || pin.len() == 6)
}
// i increments the value (initially 0)
// d decrements the value
// s squares the value
// o outputs the value into the return array
pub fn parse(code: &str) -> Vec<i32> {
    //iiisdoso
    // validate parameter
    let mut chars = code.chars();
    let commands = vec!["i", "d", "s", "o"];
    let char_validate_status = chars.any(|c| commands.contains(&c.to_string().as_str()));
    if !char_validate_status {
        panic!("invalid commands")
    }
    // _parse1(code)
    _parse2(code)
}
fn _parse1(code: &str) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut init: i32 = 0;
    code.chars().for_each(|c| {
        if c == 'i' {
            init += 1;
        } else if c == 'd' {
            init -= 1;
        } else if c == 's' {
            init = init * init;
        } else if c == 'o' {
            result.push(init);
        }
    });
    result
}
fn _parse2(code: &str) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut value: i32 = 0;
    for c in code.chars() {
        match c {
            'i' => value += 1,
            'd' => value -= 1,
            's' => value *= value,
            'o' => result.push(value),
            _ => (),
        }
    }
    result
}
pub fn spin_words(words: &str) -> String {
    words
        .split_ascii_whitespace()
        .map(|word| {
            // solution one
            // if word.len() >= 5 {
            //     word.chars().rev().collect()
            // } else {
            //     word.to_string()
            // }
            // solution two
            match word.len() >= 5 {
                true => word.chars().rev().collect(),
                false => word.to_string(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
pub fn perimeter(n: u64) -> u64 {
    let mut map = HashMap::new();
    (1..n + 2)
        .map(|item| fibonacci(item, &mut map))
        .sum::<u64>()
        * 4
}
// elegant solution
pub fn perimeter2(n: u64) -> u64 {
    (0..n)
        .fold((1u64, 1u64, 1u64), |a, _| (a.1, a.0 + a.1, a.1 + a.2))
        .2
        * 4
}

fn fibonacci(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    match map.get(&n) {
        Some(v) => *v,
        None => {
            let res = match n {
                0 => 0,
                1 => 1,
                2 => 1,
                n => fibonacci(n - 1, map) + fibonacci(n - 2, map),
            };
            map.entry(n).or_insert(res);
            return res;
        }
    }
}
pub fn order(sentence: &str) -> String {
    if sentence.len() == 0 {
        String::from("")
    } else {
        // let length = sentence.split_ascii_whitespace().count();
        // let mut array: [u64; length];
        let mut map = HashMap::new();

        sentence.split_ascii_whitespace().for_each(|s| {
            s.chars().for_each(|c| match c.to_digit(10) {
                // Some(v) => vec.set = s,
                Some(k) => {
                    map.insert(k as usize, s.to_string());
                }
                _ => (),
            });
        });
        let mut res = Vec::new();
        for k in 1..sentence.len() {
            match map.get(&k) {
                Some(v) => res.push(v.to_string()),
                None => (),
            }
        }
        res.join(" ")
    }
}
pub fn high(input: &str) -> &str {
    input
        .split_ascii_whitespace()
        .rev()
        .max_by_key(|s| calculate_value(s))
        .unwrap_or("")
    //    let mut max: u16 = 0;
    // let mut max_str = "";
    // input.split_ascii_whitespace().for_each(|s| {
    //     let total = calculate_value(s);
    //     if total > max {
    //         max = total;
    //         max_str = s;
    //     }
    // });
    // max_str
}
fn calculate_value(str: &str) -> u16 {
    // str.chars().fold(0, |sum, item| sum + item as u32 - 96)
    str.chars().map(|s| s as u16 - 96).sum::<u16>()
}
pub fn find_missing_letter(chars: &[char]) -> char {
    let mut missing_char = 'a';
    chars.iter().fold(chars[0] as char, |prev, curr| {
        if (*curr as u8) - (prev as u8) != 1 {
            missing_char = (prev as u8 + 1) as char;
        }
        *curr
    });
    missing_char
}
pub fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let wmap = get_char_number(word);
    words.iter().fold(Vec::new(), |mut strs, curr| {
        let map = get_char_number(&curr);
        map.iter().for_each(|item| match wmap.get(item.0) {
            Some(v) => {
                if item.1 == v {
                    strs.push(item.1)
                }
            }
            None => (),
        });

        return strs;
    });
    // println!("{:#?}", maps);

    vec!["a".to_string()]
}
fn get_char_number(word: &str) -> HashMap<char, i32> {
    let init_map = HashMap::new();
    word.chars().fold(init_map, |mut map, char| {
        let count = map.entry(char).or_insert(0);
        *count += 1;
        return map;
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!(
            in_array(
                &["xyz", "live", "strong"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["live", "strong"]
        );

        assert_eq!(
            in_array(
                &["live", "strong", "arp"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["arp", "live", "strong"]
        );

        assert_eq!(
            in_array(
                &["tarp", "mice", "bull"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            [] as [&str; 0]
        );

        assert_eq!(
            in_array(
                &["live", "strong", "arp", "arp"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["arp", "live", "strong"]
        );
        assert_eq!(
            in_array(
                &["live", "strong", "arp", "arp", "e", "e"],
                &["lively", "alive", "harp", "sharp", "armstrong", "e"],
            ),
            ["arp", "e", "live", "strong"]
        );
        assert_eq!(
            in_array(
                &["hen", "acus", "e", "ac", "feugi", "e", "qu", "psum", "Lor", "tel"],
                &[
                    "id",
                    "lacus",
                    "vitae",
                    "tellus",
                    "gravida",
                    "vestibulum",
                    "erat",
                    "ullamcorper.",
                    "Morbi"
                ],
            ),
            ["ac", "acus", "e", "tel"],
        );
    }

    #[test]
    fn returns_expected() {
        assert_eq!(song_decoder("WUBAWUBWUBC"), "A C"); // WUB A WUB WUB C
        assert_eq!(song_decoder("AWUBWUBWUBBWUBWUBWUBC"), "A B C"); // A WUB WUB WUB B WUB WUB WUB C
        assert_eq!(song_decoder("WUBAWUBBWUBCWUB"), "A B C");
        assert_eq!(song_decoder("AWUBBWUBC"), "A B C");
    }
    #[test]
    fn sample_tests() {
        assert_eq!(super::persistence(39), 3);
        assert_eq!(super::persistence(4), 0);
        assert_eq!(super::persistence(25), 2);
        assert_eq!(super::persistence(999), 4);

        assert_eq!(super::persistence2(39), 3);
        assert_eq!(super::persistence2(4), 0);
        assert_eq!(super::persistence2(25), 2);
        assert_eq!(super::persistence2(999), 4);
    }
    #[test]
    fn invalid_length_tests() {
        assert_eq!(validate_pin("1"), false);
        assert_eq!(validate_pin("12"), false);
        assert_eq!(validate_pin("123"), false);
        assert_eq!(validate_pin("12345"), false);
        assert_eq!(validate_pin("1234567"), false);
        assert_eq!(validate_pin("-1234"), false);
        assert_eq!(validate_pin("1.234"), false);
        assert_eq!(validate_pin("-1.234"), false);
        assert_eq!(validate_pin("00000000"), false);
    }
    #[test]
    fn non_digit_chars_tests() {
        assert_eq!(validate_pin("a234"), false);
        assert_eq!(validate_pin(".234"), false);
    }
    #[test]
    fn valid_pin_tests() {
        assert_eq!(validate_pin("1234"), true);
        assert_eq!(validate_pin("0000"), true);
        assert_eq!(validate_pin("1111"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("098765"), true);
        assert_eq!(validate_pin("000000"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("090909"), true);
    }
    #[test]
    fn parse_tests() {
        assert_eq!(parse("iiisdoso"), vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
    }
    #[test]
    fn tests_spin_words() {
        assert_eq!(spin_words("Welcome"), "emocleW");
        assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
        assert_eq!(spin_words("This is a test"), "This is a test");
        assert_eq!(spin_words("This is another test"), "This is rehtona test");
        assert_eq!(
            spin_words("You are almost to the last test"),
            "You are tsomla to the last test"
        );
        assert_eq!(
            spin_words("Just kidding there is still one more"),
            "Just gniddik ereht is llits one more"
        );
        assert_eq!(
            spin_words("Seriously this is the last one"),
            "ylsuoireS this is the last one"
        );
    }
    fn dotest(n: u64, exp: u64) -> () {
        // assert_eq!(perimeter(n), exp);
        assert_eq!(perimeter2(n), exp);
    }

    #[test]
    fn basics_perimeter() {
        dotest(5, 80);
        dotest(7, 216);
        dotest(20, 114624);
        dotest(30, 14098308);
    }
    #[test]
    fn test_order() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
    #[test]
    fn test_high() {
        assert_eq!(high("man i need a taxi up to ubud"), "taxi");
        assert_eq!(high("what time are we climbing up the volcano"), "volcano");
        assert_eq!(high("take me to semynak"), "semynak");
        assert_eq!(high("massage yes massage yes massage"), "massage");
        assert_eq!(high("take two bintang and a dance please"), "bintang");
        assert_eq!(high("aa b"), "aa");
        assert_eq!(high("b aa"), "b");
        assert_eq!(high("bb d"), "bb");
        assert_eq!(high("d bb"), "d");
        assert_eq!(high("aaa b"), "aaa");
    }
    #[test]
    fn example_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }
    #[test]
    fn tests_anagrams() {
        do_test("abba", &["aabb", "abcd", "bbaa", "dada"], &["aabb", "bbaa"]);

        do_test(
            "racer",
            &["crazer", "carer", "racar", "caers", "racer"],
            &["carer", "racer"],
        );
    }

    fn do_test(word: &str, words: &[&str], exp: &[&str]) {
        let words: Vec<String> = words.iter().map(|w| w.to_string()).collect();
        let expected: Vec<String> = exp.iter().map(|w| w.to_string()).collect();
        let got = anagrams(word, &words);
        assert_eq!(
            got, expected,
            "Failed with word: \"{}\"\nwords: {:#?}",
            word, words
        );
    }
}
