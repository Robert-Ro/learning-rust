use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    let field_name = String::from("Favoriate color");
    let field_value = String::from("Blue");
    scores.insert(field_name, 20); //expected integer, found struct `String`

    // println!("{}", field_name); // value borrowed here after move 丢失所有权
    print_hash_map(scores);
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:#?}", score); // Some(10)

    // print_hash_map(scores); error

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    scores.insert(String::from("Red"), 100);

    // print_hash_map(scores);

    scores.entry(String::from("Red")).or_insert(111);
    scores.entry(String::from("Yellow")).or_insert(111);

    print_hash_map(scores);

    let text = "Hello World wonderful World";
    let mut map = HashMap::new();
    map.insert("Hello", 1);
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // &mut V
        *count += 1; // * 解引用
    }
    println!("{:#?}", map);
}

fn print_hash_map(hash: HashMap<String, i32>) {
    for (key, value) in &hash {
        println!("{}: {}", key, value);
    }
}
