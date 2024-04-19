fn max_value(list: &[i32]) -> &i32 {
    let mut current_max = &list[0];

    for item in list {
        if item > current_max {
            current_max = item;
        }
    }
    current_max
}

fn main() {
    let number_list = vec![45, 76, 12, 0, 34, 8];

    let result = max_value(&number_list);
    println!("Maximum value: {}", result);
}
