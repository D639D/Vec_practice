use std::collections::HashMap;

fn main() {
    let mut numbers = vec![3, 2, 3, 1, 1];
    println!("{}", median(&mut numbers));
    println!("{}", mode_of_list(numbers));
}

fn median(numbers: &mut Vec<u32>) -> u32 {
    numbers.sort();
    let mut tmp = numbers[0];
    for i in 0..numbers.len() / 2 {
        tmp = numbers[i];
    }
    tmp
}

fn mode_of_list(numbers: Vec<u32>) -> u32 {
    let mut counts = HashMap::new();
    for item in numbers {
        let count = counts.entry(item).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    let mut mode = 0;
    for (key, value) in counts {
        if max_count < value {
            max_count = value;
            mode = key;
        }
    }
    mode
}
