use std::collections::HashMap;
trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0);
    }
}
struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0);
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // 不可变借用
    v.push(6); //可变借用

    // println!("the first element is: {}", first); 数组对应的内存可能已经改变了

    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];
    for i in v {
        i.display();
    }
    // vec to hashmap
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("俄罗斯队".to_string(), 50),
    ];
    // 委托编译器推导类型
    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
    // let teams_map = teams_list.into_iter().collect();  consider giving `teams_map` a type
    println!("{:?}", teams_map);

    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys: HashMap<&str, u8> = HashMap::new();

    handsome_boys.insert(&name, age);
    handsome_boys.insert("leilei", 20);

    println!("{}, {}, {:?}", name, age, handsome_boys);
    for (k, v) in &handsome_boys {
        println!("k is: {}, v is: {}", k, v);
    }
}
