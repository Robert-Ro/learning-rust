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
}
