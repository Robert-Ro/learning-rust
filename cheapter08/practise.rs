use std::collections::HashMap;
fn main() {
    let mut arr = vec![1, 2, 3, 4, 4, 6, 5, 17, 9, 11, 5, 2];
    let average = get_average(&arr);
    println!("the average of {:#?} is: {}", arr, average);
    println!("the max time value is: {}", get_total_times(&arr));

    arr.sort();
    println!("中位数是: {}", get_median(&arr));
}

fn get_average(arr: &Vec<i32>) -> f32 {
    let mut sum = 0;
    let len = arr.len() as f32;
    for elem in arr {
        sum += elem;
    }
    (sum as f32) / len
}
fn get_median(arr: &Vec<i32>) -> f32 {
    let len = arr.len();
    let median = match len % 2 {
        1 => arr[len / 2] as f32,
        _ => (arr[len / 2 - 1] + arr[len / 2]) as f32 / 2.0,
    };
    return median;
}
/**
 *  //FIXME 求众数
 */
fn get_total_times(arr: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for &i in arr {
        let mut count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    for (v, &k) in &map {
        if max < k {
            max = k;
        }
    }
    let mut max_value = 0;
    for (&v, &k) in &map {
        if max == k {
            max_value = v;
        } else {
            max_value = -1;
        }
    }
    max_value
}
