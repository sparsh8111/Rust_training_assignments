fn main() {
    let numbers = vec![12,16,15,19];
    let max_value = get_max_value(&numbers);

    match max_value {
        Some(max) => println!("Max value: {}", max),
        None => println!("empty"),
    }
}

fn get_max_value(vector: &Vec<i32>) -> Option<i32> {
    if vector.is_empty() {
        None // Handle the case where the vector is empty

    } else {
        Some(*vector.iter().max().unwrap())
    }
}
